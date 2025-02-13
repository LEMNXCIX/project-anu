use directories::BaseDirs;
use log::{error, info};
use serde::Serialize;
use std::fs;
use std::path::Path;

use crate::{
    common::response::ResponseComand,
    service::utils::{format_name_project, sanitize_filename},
};

#[derive(Serialize)]
pub struct FileInfo {
    name: String,
    is_dir: bool,
    path: String,
}
#[derive(Serialize, Clone, Default)]
pub struct DirectoryEntry {
    path: String,
    name: String,
    is_directory: bool,
}
#[derive(Serialize, Clone)]
pub struct DirectoryListing {
    path: String,
    entries: Vec<DirectoryEntry>,
}

#[tauri::command]
pub fn list_directory(path: &str) -> ResponseComand {
    info!("Listing directory: {}", path);
    let mut response = ResponseComand::new();
    let mut entries = Vec::new();
    let path = Path::new(path);
    // if !path.exists() || !path.is_dir() {
    //     return Err("El directorio especificado no existe o no es un directorio.".to_string());
    // }

    let base_dirs = match BaseDirs::new() {
        Some(dirs) => dirs,
        None => {
            response.message = "Could not get base directories".to_string();
            response.error = true;
            return response;
        }
    };

    // Crear el path completo para "Repositories" dentro de la carpeta de descargas
    let repo_path = base_dirs
        .home_dir()
        .join("Downloads")
        .join("Repositories")
        .join(path);
    info!("Obteniendo contenido del directorio: {:?}", repo_path);

    // Lee el contenido del directorio
    match fs::read_dir(repo_path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                match entry {
                    Ok(entry) => {
                        let file_type = entry.file_type().unwrap();
                        let entry_path = entry.path();
                        entries.push(DirectoryEntry {
                            path: entry_path.to_string_lossy().into_owned().to_string(),
                            name: entry_path
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned()
                                .to_string(),
                            is_directory: file_type.is_dir(),
                        });
                    }
                    Err(e) => {
                        println!("Error al leer la entrada: {}", e);
                        response.error = true;
                        response.message = e.to_string();
                    }
                }
            }
        }
        Err(e) => {
            println!("Error al leer el directorio: {}", e);
            response.message = e.to_string();
            response.error = true;

            return response;
        }
    }
    response.success = true;
    response.data = serde_json::to_value(DirectoryListing {
        path: path.to_str().unwrap_or("").to_string(),
        entries,
    })
    .unwrap();
    response
}
//#[tauri::command]

pub fn create_directory(path: &str) -> ResponseComand {
    let mut response = ResponseComand::new();
    info!("Creando carpeta para: {}", path);

    //Se formatea el nombre del proyecto
    let name_formated = format_name_project(path);
    info!("Nombre formateado: {:?}", name_formated);
    if name_formated.is_err() {
        let error_message = name_formated.err().unwrap();
        error!("Error al crear el directorio: {}", error_message);
        response.message = error_message;
        return response;
    }
    let name_formated = sanitize_filename(name_formated.unwrap().as_str());
    if name_formated.is_err() {
        let error_message = name_formated.err().unwrap();
        error!("Error al crear el directorio: {}", error_message);
        response.message = error_message;
        return response;
    }

    //
    info!("Creando directorio: {:?}", name_formated);
    let base_dirs = match BaseDirs::new() {
        Some(dirs) => dirs,
        None => {
            response.message = "Could not get base directories".to_string();
            response.error = true;
            return response;
        }
    };

    // Se crea el path completo para "Repositories" dentro de la carpeta de descargas
    let repo_path = base_dirs
        .home_dir()
        .join("Downloads")
        .join("Repositories")
        .join(&name_formated.unwrap());

    //Se verifica si el directorio ya existe
    let path = Path::new(&repo_path);
    if path.exists() {
        response.message = "El directorio especificado ya existe.".to_string();
        return response;
    }
    //Se crea el directorio
    info!("Creando directorio: {}", path.display());
    if let Err(e) = fs::create_dir(path) {
        error!("Error al crear el directorio: {}", e);
        response.message = e.to_string();
        return response;
    }

    response.success = true;
    response.message = "Directorio creado con éxito.".to_string();
    response
}

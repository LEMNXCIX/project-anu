use directories::BaseDirs;
use log::{error, info};
use serde::Serialize;
use std::fs;
use std::path::Path;

use crate::utils::{format_name_project, sanitize_filename};

#[derive(Serialize)]
pub struct FileInfo {
    name: String,
    is_dir: bool,
    path: String,
}
#[derive(Serialize, Clone)]
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
pub fn list_directory(path: &str) -> Result<DirectoryListing, String> {
    info!("Listing directory: {}", path);
    let mut entries = Vec::new();
    let path = Path::new(path);
    // if !path.exists() || !path.is_dir() {
    //     return Err("El directorio especificado no existe o no es un directorio.".to_string());
    // }

    let base_dirs = BaseDirs::new().ok_or("Could not get base directories")?;
    let download_dir = base_dirs.home_dir().join("Downloads");
    info!("Download directory: {:?}", download_dir);

    // Crear el path completo para "Repositories" dentro de la carpeta de descargas
    let repo_path = download_dir.join("Repositories").join(path);
    info!("Repositories directory: {:?}", repo_path);

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
                    Err(e) => println!("Error al leer la entrada: {}", e),
                }
            }
        }
        Err(e) => {
            println!("Error al leer el directorio: {}", e);
            return Err(e.to_string());
        }
    }

    Ok(DirectoryListing {
        path: path.to_str().unwrap_or("").to_string(),
        entries,
    })
}
#[tauri::command]
pub fn create_directory(path: &str) -> Result<(), String> {
    log::info!("Creando carpeta para: {}", path);
    let mut name = match format_name_project(path) {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    name = sanitize_filename(&name);
    info!("Creando directorio: {}", name);
    //CREAR EL DIRECTORIO EN UNA RUTA ESPECIFICA
    let path = Path::new(&name);
    if path.exists() {
        return Err("El directorio especificado ya existe.".to_string());
    }
    let base_dirs = BaseDirs::new().ok_or("Could not get base directories")?;
    let download_dir = base_dirs.home_dir().join("Downloads");
    info!("Download directory: {:?}", download_dir);

    // Crear el path completo para "Repositories" dentro de la carpeta de descargas
    let repo_path = download_dir.join("Repositories").join(path);
    info!("Repositories directory: {:?}", repo_path);
    let path = Path::new(&repo_path);
    info!("Creando directorio: {}", path.display());
    if path.exists() {
        return Err("El directorio especificado ya existe.".to_string());
    }
    //CREAR EL DIRECTORIO EN C:\Users\NUC 12\Downloads\Repositories

    if let Err(e) = fs::create_dir(path) {
        error!("Error al crear el directorio: {}", e);
        return Err(format!("Error al crear el directorio: {}", e));
    }

    Ok(())
}

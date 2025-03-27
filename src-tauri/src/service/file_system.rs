use log::{error, info};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::service::config_user::get_item;
use crate::{
    common::response::ApiResponse,
    common::utils::{format_name_project, sanitize_filename},
    service::models::file_system::{DirectoryEntry, DirectoryListing},
};

/// Lista el contenido de un directorio específico dentro de "Repositories" en Downloads.
///
/// # Argumentos
/// * `path` - Ruta relativa o nombre del directorio a listar dentro de Repositories.
///
/// # Retorno
/// Un `ApiResponse` con el estado de la operación y un `DirectoryListing` en `data` si es exitoso.
///
/// # Errores
/// - Fallo al obtener los directorios base del sistema.
/// - Error al leer el directorio especificado.
/// - Problemas con entradas individuales en el directorio.
pub fn list_directory(path: &str) -> ApiResponse {
    info!("Listing directory: {}", path);  

    let path = Path::new(&path);
    info!("Obteniendo contenido del directorio: {:?}", path);
    // Validar que la ruta sea un directorio
    if !path.exists() {
        error!("El directorio no existe: {}", path.display());
        return ApiResponse::new_error(
            format!("El directorio no existe: {}", path.display()),
            vec![],
        );
    }
    if !path.is_dir() {
        error!("La ruta no es un directorio: {}", path.display());
        return ApiResponse::new_error(
            format!("La ruta no es un directorio: {}", path.display()),
            vec![],
        );
    }

    // Leer el contenido del directorio
    let mut entries = Vec::new();
    match fs::read_dir(path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                match entry {
                    Ok(entry) => {
                        let file_type = match entry.file_type() {
                            Ok(ft) => ft,
                            Err(e) => {
                                error!("Error al obtener el tipo de archivo: {}", e);
                                continue; // Saltar esta entrada y continuar
                            }
                        };
                        let entry_path = entry.path();
                        entries.push(DirectoryEntry {
                            path: entry_path.to_string_lossy().into_owned(),
                            name: entry_path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_default(),
                            is_directory: file_type.is_dir(),
                        });
                    }
                    Err(e) => {
                        error!("Error al leer la entrada: {}", e);
                        continue; // Continuar con la siguiente entrada en caso de error
                    }
                }
            }
        }
        Err(e) => {
            error!("Error al leer el directorio: {}", e);
            return ApiResponse::new_error(
                format!("Error al leer el directorio: {}", e),
                vec![],
            );
        }
    }

    ApiResponse::new_success(
        serde_json::to_value(DirectoryListing {
            path: path.to_string_lossy().into_owned(),
            entries,
        }).unwrap_or(serde_json::Value::Null),
        "Directorio listado con éxito".to_string(),
    )
}

/// Crea un directorio dentro de "Repositories" en Downloads basado en un nombre formateado.
///
/// # Argumentos
/// * `path` - Nombre o ruta base para formatear y usar como nombre del directorio.
///
/// # Retorno
/// Un `ApiResponse` con el estado de la operación y la ruta creada en `data` si es exitoso.
///
/// # Errores
/// - Fallo al obtener los directorios base del sistema.
/// - Error al formatear o sanitizar el nombre.
/// - Directorio ya existente.
/// - Error al crear el directorio.
pub fn create_directory(path: &str) -> ApiResponse {
    info!("Creando carpeta para: {}", path);

    // Formatear el nombre del proyecto
    let name_formated = match format_name_project(path) {
        Ok(name) => name,
        Err(e) => {
            error!("Error al formatear nombre: {}", e);
            return ApiResponse::new_error(
                e.clone(),
                vec![format!("Error al formatear nombre: {}", e)],
            );
        }
    };

    // Sanitizar el nombre del directorio
    let sanitized_name = match sanitize_filename(name_formated.as_str()) {
        Ok(name) => name,
        Err(e) => {
            error!("Error al sanitizar nombre: {}", e);
            return ApiResponse::new_error(
                e.clone(),
                vec![format!("Error al sanitizar nombre: {}", e)],
            );
        }
    };

    info!("Creando directorio: {:?}", sanitized_name);

    // Obtener directorios base
    let config_file_ruta = get_item("ruta_base".to_string());
    let ruta_base = config_file_ruta.get_data().as_str().unwrap().to_string();
    // para windows la la barra es invertida
    let mut ruta_proyecto = ruta_base + "/" + &sanitized_name;
    if cfg!(windows) {
        ruta_proyecto = ruta_proyecto.replace("/", "\\");
    }

    let path = Path::new(&ruta_proyecto);

    // Verificar si el directorio existe
    if path.exists() {
        info!("Directorio ya existe: {}", path.display());
        return ApiResponse::new_warning(
            "Ya existe un directorio creado para el proyecto.".to_string(),
            vec![format!("Directorio ya existe en: {}", path.display())],
        );
    }

    // Crear el directorio
    info!("Creando directorio: {}", path.display());
    if let Err(e) = fs::create_dir(path) {
        error!("Error al crear el directorio: {}", e);
        return ApiResponse::new_error(
            format!("Error al crear el directorio: {}", e),
            vec![],
        );
    }

    ApiResponse::new_success(
        serde_json::json!(path.display().to_string()),
        "Directorio para el proyecto creado con éxito.".to_string(),
    )
}


pub fn write_file(path: &str, content: &str) -> ApiResponse {
    info!("Escribiendo archivo: {}", path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            error!("Error al abrir archivo para escritura: {}", e);
            return ApiResponse::new_error(
                "Error al abrir archivo para escritura".to_string(),
                vec![e.to_string()],
            );
        }
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        error!("Error al escribir archivo: {}", e);
        return ApiResponse::new_error(
            "Error al escribir archivo".to_string(),
            vec![e.to_string()],
        );
    }

    ApiResponse::new_success(serde_json::json!(content), "Operación completada exitosamente".to_string())
}

pub fn list_directory_by_proyect_name(project_name: &str) -> ApiResponse {
    info!("Listando directorios del proyecto {}", project_name);
    let config_file_ruta = get_item("ruta_base".to_string());
    let ruta_base = config_file_ruta.get_data().as_str().unwrap().to_string();
    let ruta_proyecto = ruta_base+ "/" + project_name;

    let path = Path::new(&ruta_proyecto);

    if!path.is_dir()
    {
        return ApiResponse::new_error(
            "El directorio no existe".to_string(),
            vec![format!("El directorio no existe: {}", path.display())],
        );
    }

    // Verificar si el directorio existe
    let encontrados = list_directory(path.to_str().unwrap());

    return encontrados;
}
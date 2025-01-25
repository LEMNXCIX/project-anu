use crate::service::{
    file_system::{create_directory, list_directory},
    utils::format_name_project,
};
use log::{error, info};

#[tauri::command]
pub fn format_name_project_command(name: &str) -> Result<String, String> {
    info!("Formateando el nombre del proyecto: {}", name);
    let name = match format_name_project(name) {
        Ok(name) => name,
        Err(e) => {
            error!("Error al formatear el nombre del proyecto: {}", e);
            return Err(e);
        }
    };
    info!("Nombre formateado: {}", name);
    Ok(name)
}

#[tauri::command]
pub fn create_directory_command(name: &str) -> Result<String, String> {
    let name_response = create_directory(name);
    if name_response.error {
        let error_message = &name_response.message;
        error!("Error al crear el directorio: {}", error_message);
        return Err(serde_json::to_string(&name_response).unwrap());
    }

    Ok(serde_json::to_string(&name_response).unwrap())
}

#[tauri::command]
pub fn list_directory_command(name: &str) -> Result<String, String> {
    let name_response = list_directory(name);
    if name_response.error {
        let error_message = &name_response.message;
        error!("Error al listar el directorio: {}", error_message);
        return Err(serde_json::to_string(&name_response).unwrap());
    }

    Ok(serde_json::to_string(&name_response).unwrap())
}

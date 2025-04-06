use crate::shared::utils::format_name_project;
use log::error;
use tauri::command;

#[command]
pub fn format_name_project_command(name: &str) -> Result<String, String> {
    let name = match format_name_project(name) {
        Ok(name) => name,
        Err(e) => {
            error!("Error al formatear el nombre del proyecto: {}", e);
            return Err(e);
        }
    };
    Ok(name)
}
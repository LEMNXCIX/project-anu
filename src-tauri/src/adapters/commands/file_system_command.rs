use crate::shared::response::ApiResponse;
use crate::use_cases::file_system::{
    create_directory, list_directory, list_directory_by_proyect_name,
};

use tauri::command;

#[command]
pub fn create_directory_command(name: &str) -> ApiResponse {
    return create_directory(name);
}

#[command]
pub fn list_directory_command(name: &str) -> ApiResponse {
    return list_directory(name);
}

#[tauri::command]
pub fn list_directory_by_proyect_name_command(name: &str) -> ApiResponse {
    return list_directory_by_proyect_name(name);
}

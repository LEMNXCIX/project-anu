use crate::common::response::ApiResponse;
use crate::service::file_system::{
    create_directory, list_directory, list_directory_by_proyect_name,
};
use dirs::download_dir;
use log::{error, info};
use tauri::command;
use tauri_plugin_dialog::FileDialogBuilder;

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

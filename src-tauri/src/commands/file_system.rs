use crate::common::response::ApiResponse;
use crate::service::file_system::{create_directory, list_directory};
use tauri::command;

#[command]
pub fn create_directory_command(name: &str) -> Result<ApiResponse, String> {
    let response = create_directory(name);
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}

#[command]
pub fn list_directory_command(name: &str) -> Result<ApiResponse, String> {
    let response = list_directory(name);
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}

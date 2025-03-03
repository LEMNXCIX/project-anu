use tauri::command;
use crate::service::updater::{check_for_updates, apply_update};
use crate::common::response::ApiResponse;

#[command]
pub async fn check_updates_comand(app_handle: tauri::AppHandle) -> Result<ApiResponse, String> {
    let response = check_for_updates(app_handle).await;
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}

#[command]
pub async fn apply_update_comand(app_handle: tauri::AppHandle) -> Result<ApiResponse, String> {
    let response = apply_update(app_handle).await;
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}
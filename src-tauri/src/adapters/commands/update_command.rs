use crate::shared::response::ApiResponse;
use crate::use_cases::updater::{apply_update, check_for_updates};
use tauri::command;

#[command]
pub async fn check_updates_comand(app_handle: tauri::AppHandle) -> ApiResponse {
    return check_for_updates(app_handle).await;
}

#[command]
pub async fn apply_update_comand(app_handle: tauri::AppHandle) -> ApiResponse {
    return apply_update(app_handle).await;
}

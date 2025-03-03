use crate::common::response::ApiResponse;
use crate::service::config_user::{create_json, load_data};
use serde_json::Value;
use tauri::command;

#[command]
pub fn save_config_command(data: Value) -> ApiResponse {
    return create_json(data);
}

#[command]
pub fn load_config_command() -> ApiResponse {
    return load_data();
}

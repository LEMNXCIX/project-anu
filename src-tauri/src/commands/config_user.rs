use crate::common::response::ApiResponse;
use crate::service::config_user::{create_json, load_data};
use serde_json::Value;
use tauri::command;

#[command]
pub fn save_config_command(data: Value) -> Result<ApiResponse, String> {
    log::info!("Data received: {:?}", data);
    let response = create_json(data);
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}

#[command]
pub fn load_config_command() -> Result<ApiResponse, String> {
    let response = load_data();
    if response.is_success() {
        Ok(response)
    } else {
        Err(response.get_message().to_string())
    }
}

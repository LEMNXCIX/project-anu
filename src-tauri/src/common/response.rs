// src/api_response.rs

use log::error;
use serde::{de, Deserialize, Serialize};
use serde_json::Value;

/// Representa una respuesta genérica de la API para comandos Tauri.
/// Contiene el estado de éxito/error, datos opcionales, un mensaje y detalles de error.
///
/// # Ejemplo
/// ```rust
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    success: bool,
    error: bool,
    data: Value,
    message: String,
    error_details: Vec<String>,
}

impl ApiResponse {
    pub fn new() -> Self {
        ApiResponse {
            success: false,
            error: false,
            data: Value::Null,
            message: String::new(),
            error_details: Vec::new(),
        }
    }

    pub fn new_success(data: Value, message: String) -> Self {
        ApiResponse {
            success: true,
            error: false,
            data,
            message,
            error_details: Vec::new(),
        }
    }
    pub fn new_warning(message: String, details: Vec<String>) -> Self {
        ApiResponse {
            success: false,
            error: false,
            data: Value::Null,
            message,
            error_details: details,
        }
    }
    pub fn new_error(message: String, details: Vec<String>) -> Self {
        ApiResponse {
            success: false,
            error: true,
            data: Value::Null,
            message,
            error_details: details,
        }
    }

    pub fn add_error_detail(&mut self, detail: String) {
        if self.error {
            self.error_details.push(detail);
        } else {
            error!("Intentando agregar detalle de error en una respuesta de éxito");
        }
    }

    pub fn set_data(&mut self, data: Value) {
        if self.success {
            self.data = data;
        } else {
            error!("Intentando establecer datos en una respuesta no exitosa");
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_data(&self) -> &Value {
        &self.data
    }

    pub fn get_error_details(&self) -> &[String] {
        &self.error_details
    }

    pub fn is_success(&self) -> bool {
        self.success && !self.error
    }

    pub fn is_warning(&self) -> bool {
        !self.success && !self.error
    }
    pub fn is_error(&self) -> bool {
        self.error && !self.success
    }
}
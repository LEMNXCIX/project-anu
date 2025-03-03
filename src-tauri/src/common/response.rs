use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Representa una respuesta genérica de la API para comandos Tauri.
/// Contiene el estado de éxito/error, datos opcionales, un mensaje y detalles de error.
///
/// # Ejemplo
/// ```rust
/// let response = ApiResponse::new_success(serde_json::json!({"id": 1}), "Operación exitosa".to_string());
/// let error_response = ApiResponse::new_error("Fallo".to_string(), vec!["Detalle".to_string()]);
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
    /// Crea una nueva instancia de ApiResponse con valores por defecto.
    /// - `success` y `error` son `false`.
    /// - `data` es `null`.
    /// - `message` y `error_details` están vacíos.
    pub fn new() -> Self {
        ApiResponse {
            success: false,
            error: false,
            data: Value::Null,
            message: String::new(),
            error_details: Vec::new(),
        }
    }

    /// Crea una respuesta de éxito con datos y mensaje especificados.
    /// - Establece `success` a `true` y `error` a `false`.
    pub fn new_success(data: Value, message: String) -> Self {
        ApiResponse {
            success: true,
            error: false,
            data,
            message,
            error_details: Vec::new(),
        }
    }

    /// Crea una respuesta de error con mensaje y detalles especificados.
    /// - Establece `error` a `true` y `success` a `false`.
    pub fn new_error(message: String, details: Vec<String>) -> Self {
        ApiResponse {
            success: false,
            error: true,
            data: Value::Null,
            message,
            error_details: details,
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

    /// Agrega un detalle de error a la lista existente, solo si la respuesta indica error.
    pub fn add_error_detail(&mut self, detail: String) {
        if self.error {
            self.error_details.push(detail);
        } else {
            error!("Intentando agregar detalle de error en una respuesta de éxito");
        }
    }

    /// Actualiza los datos de la respuesta, solo si la respuesta indica éxito.
    pub fn set_data(&mut self, data: Value) {
        if self.success {
            self.data = data;
        } else {
            error!("Intentando establecer datos en una respuesta no exitosa");
        }
    }

    /// Obtiene el mensaje de la respuesta de forma segura.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Obtiene los datos de la respuesta de forma segura (opcional).
    pub fn get_data(&self) -> &Value {
        &self.data
    }

    /// Obtiene los detalles de error de la respuesta (vacío si no hay error).
    pub fn get_error_details(&self) -> &[String] {
        &self.error_details
    }

    /// Verifica si la respuesta indica éxito de forma estricta.
    pub fn is_success(&self) -> bool {
        self.success && !self.error
    }

    /// Verifica si la respuesta indica un error.
    pub fn is_error(&self) -> bool {
        self.error && !self.success
    }
}
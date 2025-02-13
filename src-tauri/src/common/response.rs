use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseComand {
    pub success: bool,
    pub error: bool,
    pub data: Value,
    pub message: String,
    pub error_details: Vec<String>,
}

impl ResponseComand {
    pub fn new() -> Self {
        ResponseComand {
            success: false,
            error: false,
            data: Value::Null,
            message: String::new(),
            error_details: Vec::new(),
        }
    }
}

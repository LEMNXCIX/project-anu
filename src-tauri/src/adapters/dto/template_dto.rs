// src/dto/template.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateDto {
    pub id: Option<i32>, // Opcional: si necesitas exponer el ID en la API
    pub name: String,
    pub file_id: i32,
    pub type_: String,
    pub status: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}

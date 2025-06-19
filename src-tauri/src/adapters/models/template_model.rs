// src/dto/models/template.rs
pub struct TemplateModel {
    pub id: Option<i32>,
    pub file_id: i32,
    pub name: String,
    pub type_: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

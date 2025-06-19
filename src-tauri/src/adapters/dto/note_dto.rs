use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteDto {
    pub id: Option<i32>,
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub status: String,
    pub content_format: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}
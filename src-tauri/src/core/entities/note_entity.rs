
#[derive(Clone)]
pub struct Note {
    pub id: Option<i32>,
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub content_format: String,
    pub status: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}

impl Note {
    pub fn new(
        title: String,
        content: String,
        content_format: String,
        project_id: i32,
        note_status: String,
        created_at: Option<String>,
        modified_at: Option<String>,
    ) -> Result<Self, String> {
        if title.is_empty() {
            return Err("El título no puede estar vacío".to_string());
        }
        if content.is_empty() {
            return Err("El contenido no puede estar vacío".to_string());
        }
        if !["markdown", "html"].contains(&content_format.as_str()) {
            return Err("Formato de contenido inválido".to_string());
        }
        Ok(Note {
            id: None,
            project_id,
            title,
            content,
            content_format,
            status: note_status,
            created_at,
            modified_at,
        })
    }
}
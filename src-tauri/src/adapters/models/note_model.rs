
pub struct NoteModel {
    pub id: Option<i32>,
    pub project_id: i32,
    pub title: String,
    pub content: String,
    pub content_format: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

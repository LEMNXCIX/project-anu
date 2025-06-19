
use crate::core::entities::Note;
pub trait NoteRepository {
    fn get_all(&mut self) -> Result<Vec<Note>, String>;
    fn create(&mut self, note: &Note) -> Result<(), String>;
    fn delete(&mut self, note_id: i32) -> Result<(), String>;
    fn find_by_name(&mut self, note_name: &str) -> Result<Vec<Note>, String>;
    fn find_by_title(&mut self, note_title: &str) -> Result<Vec<Note>, String>;
    fn find_by_project_id(&mut self, project_id: i32) -> Result<Vec<Note>, String>;
    fn find_by_id(&mut self, note_id: i32) -> Result<Option<Note>, String>;
    fn update(&mut self, note: &Note) -> Result<(), String>;
}
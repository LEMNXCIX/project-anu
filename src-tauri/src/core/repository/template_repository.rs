use crate::core::entities::Template;

pub trait TemplateRepository {
    fn get_all(&mut self) -> Result<Vec<Template>, String>;
    fn create(&mut self, template: &Template) -> Result<(), String>;
    fn delete(&mut self, template_id: i32) -> Result<(), String>;
    fn find_by_name(&mut self, template_name: &str) -> Result<Vec<Template>, String>;
}

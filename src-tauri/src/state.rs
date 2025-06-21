use std::sync::{Arc, Mutex};
use crate::core::repository::template_repository::TemplateRepository;

pub struct AppState {
    pub template_repository: Arc<Mutex<Box<dyn TemplateRepository + Send>>>,
}
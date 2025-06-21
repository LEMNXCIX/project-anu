use chrono::NaiveDateTime;

use crate::core::entities::Template;

pub struct TemplateModel {
    pub id: Option<i32>,
    pub file_id: i32,
    pub name: String,
    pub type_: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

impl From<Template> for TemplateModel {
    fn from(entity: Template) -> Self {
        Self {
            id: None,
            name: entity.name,
            file_id: entity.file_id,
            type_: entity.type_,
            status: entity.status,
            created_at: None,
            modified_at: None,
        }
    }
}

impl From<TemplateModel> for Template {
    fn from(model: TemplateModel) -> Self {
        Self {
            name: model.name,
            file_id: model.file_id,
            type_: model.type_,
            status: model.status,
        }
    }
}
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::core::entities::Template;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateDto {
    pub id: Option<i32>,
    pub name: String,
    pub file_id: i32,
    pub type_: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

impl TryFrom<TemplateDto> for Template {
    type Error = String;

    fn try_from(dto: TemplateDto) -> Result<Self, Self::Error> {
        Template::new(dto.name, dto.file_id, dto.type_, dto.status)
    }
}

impl From<Template> for TemplateDto {
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
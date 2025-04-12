// src/dto/mappers/template.rs
use crate::adapters::dto::template_dto::TemplateDto;
use crate::adapters::models::TemplateModel;
use crate::core::entities::Template;

pub fn model_to_entity(model: TemplateModel) -> Template {
    Template {
        name: model.name,
        path: model.path,
        type_: model.type_,
        status: model.status,
    }
}

pub fn entity_to_model(entity: Template) -> TemplateModel {
    TemplateModel {
        id: None,
        name: entity.name,
        path: entity.path,
        type_: entity.type_,
        status: entity.status,
        created_at: None,
        modified_at: None,
    }
}

pub fn model_to_dto(model: TemplateModel) -> TemplateDto {
    TemplateDto {
        id: model.id,
        name: model.name,
        path: model.path,
        type_: model.type_,
        status: model.status,
        created_at: model.created_at.map(|d| d.to_string()),
        modified_at: model.modified_at.map(|d| d.to_string()),
    }
}

pub fn dto_to_entity(dto: TemplateDto) -> Template {
    Template {
        name: dto.name,
        path: dto.path,
        status: dto.status,
        type_: dto.type_,
    }
}

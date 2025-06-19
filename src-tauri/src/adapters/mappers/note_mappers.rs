use crate::{adapters::{models::NoteModel, dto::note_dto::NoteDto}, core::entities::Note};

pub fn model_to_entity(model: NoteModel) -> Note {
    Note {
        id: model.id,
        project_id: model.project_id,
        title: model.title,
        content: model.content,
        content_format: model.content_format,
        status: model.status,
        created_at: model.created_at.map(|d| d.to_string()),
        modified_at: model.modified_at.map(|d| d.to_string()),
    }
}

pub fn entity_to_model(entity: Note) -> NoteModel {
    NoteModel {
        id: entity.id,
        project_id: entity.project_id,
        title: entity.title,
        content: entity.content,
        content_format: entity.content_format,
        status: entity.status,
        created_at: entity.created_at.map(|d| d.parse().ok()).flatten(),
        modified_at: entity.modified_at.map(|d| d.parse().ok()).flatten(),
    }
}

pub fn model_to_dto(model: NoteModel) -> NoteDto {
    NoteDto {
        id: model.id,
        project_id: model.project_id,
        title: model.title,
        content: model.content,
        content_format: model.content_format,
        status: model.status,
        created_at: model.created_at.map(|d| d.to_string()),
        modified_at: model.modified_at.map(|d| d.to_string()),
    }
}
pub fn dto_to_entity(dto: crate::adapters::dto::note_dto::NoteDto) -> Note {
    Note {
        id: dto.id,
        project_id: dto.project_id,
        title: dto.title,
        content: dto.content,
        content_format: dto.content_format,
        status: dto.status,
        created_at: dto.created_at.map(|d| d.to_string()),
        modified_at: dto.modified_at.map(|d| d.to_string()),
    }
}
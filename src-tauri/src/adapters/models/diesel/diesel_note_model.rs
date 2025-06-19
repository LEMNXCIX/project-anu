use crate::adapters::models::note_model::NoteModel;
use crate::schema::notes; 

use diesel::{prelude::{Insertable, Queryable, QueryableByName, Selectable}, sqlite::Sqlite};

#[derive(Queryable, Insertable, Selectable, Debug,QueryableByName)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = notes)]
pub struct DieselNote {
    pub id: Option<i32>,
    pub project_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub content_format: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl From<DieselNote> for NoteModel {
    fn from(diesel: DieselNote) -> Self {
        NoteModel {
            id: diesel.id,
            project_id: diesel.project_id,
            title: diesel.title,
            content: diesel.content.unwrap_or_default(),
            content_format: diesel.content_format,
            status: diesel.status,
            created_at: diesel.created_at,
            modified_at: diesel.modified_at,
        }
    }
}

impl From<NoteModel> for DieselNote {
    fn from(model: NoteModel) -> Self {
        DieselNote {
            id: model.id,
            project_id: model.project_id,
            title: model.title,
            content: Some(model.content),
            content_format: model.content_format,
            status: model.status,
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}


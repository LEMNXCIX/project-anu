// src/adapters/models/diesel/template.rs
use crate::adapters::models::TemplateModel;
use crate::schema::templates;
use diesel::{
    prelude::{Insertable, Queryable, QueryableByName, Selectable},
    sqlite::Sqlite,
};

#[derive(Queryable, Insertable, Selectable, Debug, QueryableByName)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(table_name = templates)]
pub struct DieselTemplate {
    pub id: Option<i32>,
    pub file_id: i32,
    pub name: String,
    pub type_: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl From<DieselTemplate> for TemplateModel {
    fn from(diesel: DieselTemplate) -> Self {
        TemplateModel {
            id: diesel.id,
            name: diesel.name,
            file_id: diesel.file_id,
            type_: diesel.type_,
            status: diesel.status,
            created_at: diesel.created_at,
            modified_at: diesel.modified_at,
        }
    }
}

impl From<TemplateModel> for DieselTemplate {
    fn from(model: TemplateModel) -> Self {
        DieselTemplate {
            id: model.id,
            name: model.name,
            file_id: model.file_id,
            type_: model.type_,
            status: model.status,
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}

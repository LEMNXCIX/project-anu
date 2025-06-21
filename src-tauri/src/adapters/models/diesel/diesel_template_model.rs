use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use chrono::NaiveDateTime;
use crate::schema::templates;

#[derive(Queryable, Insertable, Selectable, Debug, QueryableByName)]
#[diesel(table_name = templates)]
pub struct DieselTemplate {
    pub id: Option<i32>,
    pub file_id: i32,
    pub name: String,
    pub type_: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_at: Option<NaiveDateTime>,
}

impl From<DieselTemplate> for crate::adapters::models::TemplateModel {
    fn from(diesel: DieselTemplate) -> Self {
        Self {
            id: diesel.id,
            file_id: diesel.file_id,
            name: diesel.name,
            type_: diesel.type_,
            status: diesel.status,
            created_at: diesel.created_at,
            modified_at: diesel.modified_at,
        }
    }
}

impl From<crate::adapters::models::TemplateModel> for DieselTemplate {
    fn from(model: crate::adapters::models::TemplateModel) -> Self {
        Self {
            id: model.id,
            file_id: model.file_id,
            name: model.name,
            type_: model.type_,
            status: model.status,
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}
// src/adapters/models/diesel/template.rs
use crate::adapters::models::TemplateModel;
use crate::schema::templates;
use diesel::prelude::{Insertable, Queryable, Selectable};

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = templates)]
pub struct DieselTemplate {
    pub id: Option<i32>,
    pub name: String,
    pub path: String,
    pub type_: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub modified_at: Option<chrono::NaiveDateTime>,
}

impl From<DieselTemplate> for TemplateModel {
    fn from(diesel: DieselTemplate) -> Self {
        TemplateModel {
            id: diesel.id,
            name: diesel.name,
            path: diesel.path,
            type_: diesel.type_,
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
            path: model.path,
            type_: model.type_,
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}

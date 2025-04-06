use crate::adapters::mappers::template_mapper::{entity_to_model, model_to_entity};
use crate::adapters::models::diesel::DieselTemplate;
use crate::adapters::models::TemplateModel;
use crate::core::entities::template_entity::Template;
use crate::core::repository::template_repository::TemplateRepository;
use crate::infrastructure::db::connections::connection::ConnectionProvider;
use crate::schema::templates::dsl::*;
use diesel::prelude::*;

pub struct DieselTemplateRepository<T: ConnectionProvider> {
    connection_provider: T,
}

impl<T: ConnectionProvider> DieselTemplateRepository<T>
where
    T::Connection: diesel::Connection,
{
    pub fn new(connection_provider: T) -> Self {
        DieselTemplateRepository {
            connection_provider,
        }
    }
}

impl<T: ConnectionProvider> TemplateRepository for DieselTemplateRepository<T>
where
    T::Connection: diesel::Connection<Backend = diesel::sqlite::Sqlite>
        + 'static
        + diesel::connection::LoadConnection,
    templates: diesel::query_dsl::LoadQuery<'static, T::Connection, DieselTemplate>, // Para get_all
{
    fn get_all(&mut self) -> Result<Vec<Template>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = templates
            .load::<DieselTemplate>(conn)
            .map_err(|e| format!("Error al cargar datos: {}", e))?;
        let models: Vec<TemplateModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }

    fn create(&mut self, template: &Template) -> Result<(), String> {
        let conn = self.connection_provider.get_connection()?;
        let model = entity_to_model(template.clone());
        let diesel_model: DieselTemplate = model.into();
        diesel::insert_into(templates)
            .values(&diesel_model)
            .execute(conn)
            .map_err(|e| format!("Error al insertar datos: {}", e))?;
        Ok(())
    }

    fn delete(&mut self, template_id: i32) -> Result<(), String> {
        let conn = self.connection_provider.get_connection()?;
        diesel::delete(templates.filter(id.eq(template_id)))
            .execute(conn)
            .map_err(|e| format!("Error al eliminar datos: {}", e))?;
        Ok(())
    }
    fn find_by_name(&mut self, template_name: &str) -> Result<Vec<Template>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = templates
            .filter(name.eq(template_name))
            .load::<DieselTemplate>(conn)
            .map_err(|e| format!("Error al buscar datos: {}", e))?;
        let models: Vec<TemplateModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }
}

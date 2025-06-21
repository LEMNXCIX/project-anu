use crate::adapters::models::diesel::DieselTemplate;
use crate::adapters::models::TemplateModel;
use crate::core::entities::template_entity::Template;
use crate::core::repository::template_repository::TemplateRepository;
use crate::infrastructure::db::connections::connection::ConnectionProvider;
use crate::infrastructure::db::connections::sqlite::sqlite_connection::PoolConnection;
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
        DieselTemplateRepository { connection_provider }
    }
}

impl<T: ConnectionProvider> TemplateRepository for DieselTemplateRepository<T>
where
    T: Send + Sync,
    T::Connection: diesel::Connection<Backend = diesel::sqlite::Sqlite>
        + diesel::connection::LoadConnection,
{
    fn get_all(&mut self) -> Result<Vec<Template>, String> {
        let mut conn = self.connection_provider.get_connection()?;
        let diesel_models = templates
            .load::<DieselTemplate>(&mut conn)
            .map_err(|e| format!("Error al cargar datos: {}", e))?;

        Ok(diesel_models
            .into_iter()
            .map(|diesel| TemplateModel::from(diesel).into())
            .collect())
    }

    fn create(&mut self, template: &Template) -> Result<(), String> {
        let mut conn = self.connection_provider.get_connection()?;
        let model: TemplateModel = template.clone().into();
        let diesel_model: DieselTemplate = model.into();

        diesel::insert_into(templates)
            .values(&diesel_model)
            .execute(&mut conn)
            .map_err(|e| format!("Error al insertar datos: {}", e))?;

        Ok(())
    }

    fn delete(&mut self, template_id: i32) -> Result<(), String> {
        let mut conn = self.connection_provider.get_connection()?;
        diesel::delete(templates.filter(id.eq(template_id)))
            .execute(&mut conn)
            .map_err(|e| format!("Error al eliminar datos: {}", e))?;
        Ok(())
    }

    fn find_by_name(&mut self, template_name: &str) -> Result<Vec<Template>, String> {
        let mut conn = self.connection_provider.get_connection()?;
        let diesel_models = templates
            .filter(name.eq(template_name))
            .load::<DieselTemplate>(&mut conn)
            .map_err(|e| format!("Error al buscar por nombre: {}", e))?;

        Ok(diesel_models
            .into_iter()
            .map(|diesel| TemplateModel::from(diesel).into())
            .collect())
    }
}

// Asegurar que DieselTemplateRepository sea Send + Sync
unsafe impl<T: ConnectionProvider + Send + Sync> Send for DieselTemplateRepository<T> {}
unsafe impl<T: ConnectionProvider + Sync> Sync for DieselTemplateRepository<T> {}
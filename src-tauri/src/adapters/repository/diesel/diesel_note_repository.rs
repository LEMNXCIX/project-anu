use crate::adapters::mappers::note_mappers::{entity_to_model, model_to_entity};
use crate::adapters::models::diesel::DieselNote;
use crate::adapters::models::NoteModel;
use crate::core::entities::Note;
use crate::core::repository::note_repository::NoteRepository;
use crate::infrastructure::db::connections::connection::ConnectionProvider;
use crate::schema::notes::dsl::*;
use diesel::prelude::*;

pub struct DieselNoteRepository<T: ConnectionProvider> {
    connection_provider: T,
}

impl<T: ConnectionProvider> DieselNoteRepository<T>
where
    T::Connection: diesel::Connection,
{
    pub fn new(connection_provider: T) -> Self {
        DieselNoteRepository {
            connection_provider,
        }
    }
}

impl<T: ConnectionProvider> NoteRepository for DieselNoteRepository<T>
where
    T::Connection: diesel::Connection<Backend = diesel::sqlite::Sqlite>
        + 'static
        + diesel::connection::LoadConnection,
    notes: diesel::query_dsl::LoadQuery<'static, T::Connection, DieselNote>, // Para get_all
{
    fn get_all(&mut self) -> Result<Vec<Note>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = notes
            .load::<DieselNote>(conn)
            .map_err(|e| format!("Error al cargar notas: {}", e))?;
        let models: Vec<NoteModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }

    fn create(&mut self, note: &Note) -> Result<(), String> {
        let conn = self.connection_provider.get_connection()?;
        let model = entity_to_model(note.clone());
        let diesel_model: DieselNote = model.into();
        diesel::insert_into(notes)
            .values(&diesel_model)
            .execute(conn)
            .map_err(|e| format!("Error al insertar nota: {}", e))?;
        Ok(())
    }

    fn delete(&mut self, note_id: i32) -> Result<(), String> {
        let conn = self.connection_provider.get_connection()?;
        diesel::delete(notes.filter(id.eq(note_id)))
            .execute(conn)
            .map_err(|e| format!("Error al eliminar nota: {}", e))?;
        Ok(())
    }

    fn find_by_title(&mut self, note_title: &str) -> Result<Vec<Note>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = notes
            .filter(title.eq(note_title))
            .load::<DieselNote>(conn)
            .map_err(|e| format!("Error al buscar nota por título: {}", e))?;
        let models: Vec<NoteModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }

    fn find_by_project_id(&mut self, project_id_value: i32) -> Result<Vec<Note>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = notes
            .filter(project_id.eq(project_id_value))
            .load::<DieselNote>(conn)
            .map_err(|e| format!("Error al buscar notas por ID de proyecto: {}", e))?;
        let models: Vec<NoteModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }
    fn find_by_id(&mut self, note_id: i32) -> Result<Option<Note>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_model = notes
            .filter(id.eq(note_id))
            .first::<DieselNote>(conn)
            .optional()
            .map_err(|e| format!("Error al buscar nota por ID: {}", e))?;
        
        match diesel_model {
            Some(model) => {
                let note_model: NoteModel = model.into();
                Ok(Some(model_to_entity(note_model)))
            },
            None => Ok(None),
        }
    }
    fn find_by_name(&mut self, note_name: &str) -> Result<Vec<Note>, String> {
        let conn = self.connection_provider.get_connection()?;
        let diesel_models = notes
            .filter(title.eq(note_name))
            .load::<DieselNote>(conn)
            .map_err(|e| format!("Error al buscar nota por nombre: {}", e))?;
        let models: Vec<NoteModel> = diesel_models.into_iter().map(|m| m.into()).collect();
        Ok(models.into_iter().map(model_to_entity).collect())
    }
    fn update(&mut self, note: &Note) -> Result<(), String> {
        let conn = self.connection_provider.get_connection()?;
        let model = entity_to_model(note.clone());
        let diesel_model: DieselNote = model.into();
        diesel::update(notes.filter(id.eq(diesel_model.id)))
            .set((
                title.eq(&diesel_model.title),
                content.eq(&diesel_model.content),
                project_id.eq(diesel_model.project_id),
                //: Add other fields as needed
            ))
            .execute(conn)
            .map_err(|e| format!("Error al actualizar nota: {}", e))?;
        Ok(())
    }
}
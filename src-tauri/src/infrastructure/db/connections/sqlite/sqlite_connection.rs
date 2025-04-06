use crate::infrastructure::db::connections::connection::ConnectionProvider;
use diesel::connection::LoadConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, SqliteConnection};
use std::env;

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
    // Campo para almacenar temporalmente la conexión obtenida del pool
    active_connection: Option<PooledConnection<ConnectionManager<SqliteConnection>>>,
}

impl ConnectionProvider for Database {
    type Connection = SqliteConnection;
    fn get_connection(&mut self) -> Result<&mut Self::Connection, String> {
        // Si no hay una conexión activa, obtenemos una del pool
        if self.active_connection.is_none() {
            let conn = self
                .pool
                .get()
                .map_err(|e| format!("Error al obtener conexión del pool: {}", e))?;
            self.active_connection = Some(conn);
        }

        // Devolvemos una referencia mutable a la conexión subyacente
        Ok(self.active_connection.as_mut().unwrap())
    }
}

impl Database {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        let pool = Pool::builder()
            .max_size(10) // Número máximo de conexiones en el pool (ajustable)
            .build(manager)
            .expect("Error al crear el pool de conexiones");
        Database {
            pool,
            active_connection: None,
        }
    }
}

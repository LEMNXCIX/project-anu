use crate::infrastructure::db::connections::connection::ConnectionProvider;
use crate::infrastructure::db::diesel::diesel_migrations::run_migrations;
use crate::shared::constants::APP_NAME;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use log::info;
use std::{env, fs, path::Path};

pub type PoolConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        // Obtener nombre/archivo de la base de datos desde variable de entorno
        let database_name = env::var("DATABASE_URL")
            .expect("DATABASE_URL debe estar configurada");

        // Ruta completa (relativa al ejecutable actual)
        let database_path = std::env::current_dir()
            .expect("No se pudo obtener el directorio actual")
            .join(&database_name);

        // Crear el archivo si no existe
        if !database_path.exists() {
            fs::File::create(&database_path)
                .expect("No se pudo crear el archivo de la base de datos");
        }

        info!(
            "Conectando a la base de datos en: {}",
            database_path.to_string_lossy()
        );

        let database_url = format!("sqlite://{}", database_name);
        info!("URL de conexión SQLite: {}", database_url);

        // Crear pool de conexiones
        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Error al crear el pool de conexiones");

        // Ejecutar migraciones
        let mut conn = pool
            .get()
            .expect("No se pudo obtener conexión para migraciones");

        run_migrations(&mut conn);

        Database { pool }
    }

    pub fn get_connection(&self) -> Result<PoolConnection, String> {
        self.pool
            .get()
            .map_err(|e| format!("Error al obtener conexión del pool: {}", e))
    }
}

impl ConnectionProvider for Database {
    type Connection = PoolConnection;

    fn get_connection(&self) -> Result<Self::Connection, String> {
        self.get_connection()
    }
}

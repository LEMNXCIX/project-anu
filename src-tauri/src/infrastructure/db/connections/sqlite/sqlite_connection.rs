use crate::infrastructure::db::connections::connection::ConnectionProvider;
use crate::infrastructure::db::diesel::diesel_migrations::run_migrations;
use crate::shared::constants::APP_NAME;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use log::info;
use std::{env, fs};

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
        dotenvy::dotenv().ok();
        // Dentro de Database::new o donde corresponda
        let config_path = dirs::config_local_dir()
            .expect("No se pudo determinar el directorio de configuración local")
            .join(APP_NAME);

        // Lee el nombre del archivo de la base de datos desde DATABASE_URL
        let data_base_name = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");

        // Crea el directorio de configuración, NO el database_path
        fs::create_dir_all(&config_path).expect("No se pudo crear el directorio de configuración");

        // Construye la ruta completa al archivo de la base de datos
        let database_path = config_path.join(&data_base_name);

        // Log de la ruta del archivo
        info!(
            "Conectando a la base de datos en: {}",
            database_path.to_string_lossy()
        );

        // Crea la URL para SQLite con la ruta absoluta
        let database_url = format!(
            "sqlite://{}",
            database_path
                .canonicalize()
                .unwrap_or_else(|_| {
                    // Si el archivo no existe, lo creamos vacío para que canonicalize funcione después
                    fs::File::create(&database_path)
                        .expect("No se pudo crear el archivo de la base de datos");
                    database_path
                        .canonicalize()
                        .expect("No se pudo resolver la ruta de la base de datos")
                })
                .to_string_lossy()
        );

        info!("URL de conexión SQLite: {}", database_url);
        info!("Conectando a la base de datos: {}", database_url);

        let manager = ConnectionManager::<SqliteConnection>::new(
            &database_path.to_string_lossy().to_string(),
        );
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Error al crear el pool de conexiones");

        // Ejecuta migraciones al iniciar
        let mut conn = pool
            .get()
            .expect("No se pudo obtener conexión para migraciones");

        run_migrations(&mut conn);

        Database {
            pool,
            active_connection: None,
        }
    }
}

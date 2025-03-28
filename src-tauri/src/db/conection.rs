use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurado");
    SqliteConnection::establish(&database_url).expect("Error al conectar con la base de datos")
}
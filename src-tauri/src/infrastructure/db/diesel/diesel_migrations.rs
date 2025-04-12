use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// Ajusta el path a tu carpeta personalizada de migraciones
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/infrastructure/db/diesel/migrations");

pub fn run_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error al ejecutar las migraciones");
}
mod adapters;
mod core;
mod infrastructure;
mod schema;
mod shared;
mod tests;
mod use_cases;
mod state;

use adapters::commands::{
    config_user_command::{load_config_command, save_config_command},
    file_system_command::{
        create_directory_command, list_directory_by_proyect_name_command, list_directory_command, 
    },
    update_command::{apply_update_comand, check_updates_comand},
    utils_command::format_name_project_command,
    templates_comands::create_template_command,
};
use adapters::repository::diesel::diesel_template_repository::DieselTemplateRepository;
use core::repository::template_repository::TemplateRepository;
use infrastructure::{db::connections::sqlite, logging};
use logging::initialize_logger;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar el logger
    initialize_logger()?;

    // Inicializar la conexión SQLite con pool
    let db = sqlite::sqlite_connection::Database::new();

    // Crear el repositorio
    let template_repository = DieselTemplateRepository::new(db);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Box::new(template_repository) as Box<dyn TemplateRepository + Send + Sync>)
        .invoke_handler(tauri::generate_handler![
            load_config_command,
            save_config_command,
            create_directory_command,
            list_directory_command,
            list_directory_by_proyect_name_command,
            format_name_project_command,
            check_updates_comand,
            apply_update_comand,
            create_template_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
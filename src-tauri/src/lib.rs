mod adapters;
mod core;
mod infrastructure;
mod schema;
mod shared;
mod tests;
mod use_cases;

use adapters::commands::{
    config_user_command::{load_config_command, save_config_command},
    file_system_command::{
        create_directory_command, list_directory_by_proyect_name_command, list_directory_command,
    },
    update_command::{apply_update_comand, check_updates_comand},
    utils_command::format_name_project_command,
};
use infrastructure::{logging, db::connections::sqlite};
use logging::initialize_logger;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar el logger antes de cualquier otra cosa
    initialize_logger()?;
    sqlite::sqlite_connection::Database::new();
    

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_config_command,
            save_config_command,
            create_directory_command,
            list_directory_command,
            list_directory_by_proyect_name_command,
            format_name_project_command,
            check_updates_comand,
            apply_update_comand,
        ])
        //.plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

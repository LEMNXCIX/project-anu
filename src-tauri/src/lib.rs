mod commands;
mod common;
mod service;
mod logging;

use logging::initialize_logger;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar el logger antes de cualquier otra cosa
    initialize_logger()?;

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::config_user::load_config_command,
            commands::config_user::save_config_command,
            commands::file_system::create_directory_command,
            commands::file_system::list_directory_command,
            commands::file_system::list_directory_by_proyect_name_command,
            commands::utils::format_name_project_command,
            commands::update::check_updates_comand,
            commands::update::apply_update_comand,
        ])
        //.plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

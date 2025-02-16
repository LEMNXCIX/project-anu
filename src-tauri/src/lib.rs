// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod comands;
mod common;
mod service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            comands::create_directory_command,
            comands::list_directory_command,
            comands::format_name_project_command,
            comands::check_updates_comand,
            comands::apply_update_comand
        ])
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

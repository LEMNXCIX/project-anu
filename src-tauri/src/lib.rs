// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_updater::UpdaterExt;
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
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                //update(handle).await.unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    log::info!("Validando versiones nuevas");
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;
        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("Finalizo la descarga");
                },
            )
            .await?;

        println!("Se instalo la actualización");
        app.restart();
    }

    Ok(())
}

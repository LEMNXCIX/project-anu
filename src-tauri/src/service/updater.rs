use log::{error, info};
use tauri_plugin_updater::UpdaterExt;
use tauri::AppHandle;
use crate::common::response::ResponseComand;

pub async fn check_for_updates(app: AppHandle) -> ResponseComand {
    let mut response = ResponseComand::new();

    let update = app.updater();
    if update.is_err() {
        let error_message = update.err().unwrap();
        error!("Error al verificar las actualizaciones: {}", error_message);
        response.message = error_message.to_string();
        response.error = true;
        return response;
    }
    
    let update = update.unwrap().check().await;
    if let Some(update) = update.unwrap() {
        info!("Hay una actualización disponible");
        response.success = true;
        response.data = update.raw_json

    } else {
        info!("No hay actualización disponible");
        response.success = true;
        response.data = serde_json::to_value("No hay actualización disponible").unwrap();
    }

    return response;
}

pub async fn apply_update(app: AppHandle) -> ResponseComand {
    let mut response = ResponseComand::new();

    let update = app.updater();
    if update.is_err() {
        let error_message = update.err().unwrap();
        error!("Error al aplicar la actualización: {}", error_message);
        response.message = error_message.to_string();
        response.error = true;
        return response;
    }

    let update_check = app.updater();
    if update_check.is_err() {
        let error_message = update_check.err().unwrap();
        error!("Error al verificar las actualizaciones: {}", error_message);
        response.message = error_message.to_string();
        response.error = true;
        return response;
    }

    let update_check = update_check.unwrap().check().await;
    if update_check.is_err() {
        let error_message = update_check.err().unwrap();
        error!("Error al verificar las actualizaciones: {}", error_message);
        response.message = error_message.to_string();
        response.error = true;
        return response;
    }

    if let Some(update) = update_check.unwrap() {
        let mut downloaded = 0;
        // alternatively we could also call update.download() and update.install() separately
        if let Err(e) = update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    info!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    info!("Finalizo la descarga");
                },
            )
            .await
        {
            error!("Error al descargar e instalar la actualización: {}", e);
            response.message = e.to_string();
            response.error = true;
            return response;
        }

        info!("Se instalo la actualización");
        
        response.success = true;
        response.data = serde_json::to_value("Se instalo la actualización").unwrap();
        app.restart();

    } else {
        error!("No se pudo aplicar la actualización");
        response.success = true;
        response.data = serde_json::to_value("No se pudo aplicar la actualización").unwrap();
    }

    return response;
}
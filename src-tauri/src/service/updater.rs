use log::{error, info};
use tauri::AppHandle;
use crate::common::response::ApiResponse;
use tauri_plugin_updater::UpdaterExt; // Importar el trait necesario

pub async fn check_for_updates(app: AppHandle) -> ApiResponse {
    info!("Verificando actualizaciones...");

    let updater = match app.updater() {
        Ok(updater) => updater,
        Err(e) => {
            error!("Error al inicializar el updater: {}", e);
            return ApiResponse::new_error(
                "No se pudo inicializar el verificador de actualizaciones".to_string(),
                vec![e.to_string()],
            );
        }
    };

    match updater.check().await {
        Ok(Some(update)) => {
            info!("Actualización disponible: {}", update.version);
            ApiResponse::new_success(
                update.raw_json.clone(),
                "Actualización disponible".to_string(),
            )
        }
        Ok(None) => {
            info!("No hay actualizaciones disponibles");
            ApiResponse::new_success(
                serde_json::json!("No hay actualización disponible"),
                "Sin actualizaciones".to_string(),
            )
        }
        Err(e) => {
            error!("Error al verificar actualizaciones: {}", e);
            ApiResponse::new_error(
                "Fallo al verificar actualizaciones".to_string(),
                vec![e.to_string()],
            )
        }
    }
}

/// Aplica una actualización disponible para la aplicación.
///
/// # Argumentos
/// * `app` - El manejador de la aplicación Tauri.
///
/// # Retorno
/// Un `ApiResponse` con el estado de la operación y datos JSON si es exitoso.
///
/// # Errores
/// - Fallo al inicializar el updater.
/// - Error al descargar o instalar la actualización.
/// - Sin actualización disponible para aplicar.
pub async fn apply_update(app: AppHandle) -> ApiResponse {
    info!("Aplicando actualización...");

    let updater = match app.updater() {
        Ok(updater) => updater,
        Err(e) => {
            error!("Error al inicializar el updater: {}", e);
            return ApiResponse::new_error(
                "No se pudo inicializar el aplicador de actualizaciones".to_string(),
                vec![e.to_string()],
            );
        }
    };

    let update_check = match updater.check().await {
        Ok(check) => check,
        Err(e) => {
            error!("Error al verificar actualizaciones: {}", e);
            return ApiResponse::new_error(
                "Fallo al verificar actualizaciones".to_string(),
                vec![e.to_string()],
            );
        }
    };

    match update_check {
        Some(update) => {
            info!("Descargando e instalando actualización: {}", update.version);
            let mut downloaded = 0;
            if let Err(e) = update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        info!("Descargados {downloaded} de {content_length:?}");
                    },
                    || info!("Descarga finalizada"),
                )
                .await
            {
                error!("Error al descargar/instalar: {}", e);
                return ApiResponse::new_error(
                    "Fallo al descargar o instalar la actualización".to_string(),
                    vec![e.to_string()],
                );
            }

            info!("Actualización instalada con éxito");
           
            ApiResponse::new_success(
                serde_json::json!("Actualización aplicada"),
                "Actualización instalada y aplicada".to_string(),
            );

            app.restart();
        }
        None => {
            error!("No hay actualización disponible para aplicar");
            ApiResponse::new_error(
                "No se encontró actualización para aplicar".to_string(),
                vec![],
            )
        }
    }
}
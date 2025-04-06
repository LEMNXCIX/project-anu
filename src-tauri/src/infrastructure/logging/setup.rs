use dirs;
use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::{self, File};

pub fn initialize_logger() -> Result<(), std::io::Error> {
    // Obtener el directorio de configuración
    let config_dir = dirs::config_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No se pudo obtener el directorio de configuración",
        )
    })?;
    let log_dir = config_dir.join("mi_app/logs");
    fs::create_dir_all(&log_dir)?;

    // Configurar el archivo de logs
    let log_file = log_dir.join("app.log");
    let log_file = File::create(&log_file)?;

    // Configurar el formato del logger
    let config = ConfigBuilder::new().build();

    // Combinar logger de archivo y terminal
    CombinedLogger::init(vec![
        // Logger para la terminal
        TermLogger::new(
            LevelFilter::Info,   // Nivel mínimo para la terminal
            config.clone(),      // Misma configuración
            TerminalMode::Mixed, // Mixed para stdout/stderr según nivel
            ColorChoice::Always, // Siempre usar colores
        ),
        // Logger para el archivo
        WriteLogger::new(
            LevelFilter::Info, // Nivel mínimo para el archivo
            config,            // Misma configuración
            log_file,
        ),
    ])
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error al inicializar CombinedLogger: {}", e),
        )
    })?;

    info!(
        "Logger configurado, escribiendo en: {:?}",
        log_dir.join("app.log")
    );
    Ok(())
}

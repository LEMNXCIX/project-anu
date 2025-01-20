use log::error;


#[tauri::command]
pub fn format_name_project(name: &str) -> Result<String, String> {
    if name.is_empty() {
        let e = "El nombre del proyecto no puede estar vacío.".to_string();
        error!("Error al crear el directorio: {}", e);
        return Err(e);
    }
    let mut name = name.to_string();
    // Cambiar _ por espacio y - por espacio
    name = name.replace("_", ": ");
    name = name.replace("-", " ");

    Ok(name)
}

#[cfg(target_os = "windows")]
pub fn sanitize_filename(s: &str) -> String {
    s.replace('<', "-")
        .replace('>', "-")
        .replace(':', "-")
        .replace('"', "-")
        .replace('/', "-")
        .replace('\\', "-")
        .replace('|', "-")
        .replace('?', "-")
        .replace('*', "-")
}

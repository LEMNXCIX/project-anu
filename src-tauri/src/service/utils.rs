use log::error;

pub fn format_name_project(name: &str) -> Result<String, String> {
    if name.is_empty() {
        let error_message = "El nombre del proyecto no puede estar vacío.".to_string();
        error!("Error al crear el directorio: {}", error_message);
        return Err(error_message);
    }
    //Se formatea el nombre del proyecto
    let formatted_name = name.replace("_", ": ").replace("-", " ");

    Ok(formatted_name)
}
#[cfg(target_os = "windows")]
pub fn sanitize_filename(s: &str) -> Result<String, String> {
    let sanitized = s
        .replace('<', "")
        .replace('>', "")
        .replace(':', "")
        .replace('"', "")
        .replace('/', "")
        .replace('\\', "")
        .replace('|', "")
        .replace('?', "")
        .replace('*', "");
    Ok(sanitized)
}

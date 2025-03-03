use log::error;

pub fn format_name_project(name: &str) -> Result<String, String> {
    if name.is_empty() {
        let error_message = "Ingrese un valor.".to_string();
        return Err(error_message);
    }
    //Se formatea el nombre del proyecto
    let formatted_name = name.replace("_", ": ").replace("-", " ");

    Ok(formatted_name)
}

pub fn sanitize_filename(s: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Lógica específica para Windows
        let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        let sanitized = s.chars()
            .filter(|c| !invalid_chars.contains(c))
            .collect::<String>();
        Ok(sanitized)
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Lógica para otros sistemas (Linux, macOS, etc.)
        let invalid_chars = ['/']; // Ejemplo básico
        let sanitized = s.chars()
            .filter(|c| !invalid_chars.contains(c))
            .collect::<String>();
        Ok(sanitized)
    }
}

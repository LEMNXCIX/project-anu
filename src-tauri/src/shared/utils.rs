pub fn format_name_project(name: &str) -> Result<String, String> {
    if name.is_empty() {
        let error_message = "Ingrese un valor.".to_string();
        return Err(error_message);
    }

    // Divide el prefijo del resto con el primer guion bajo
    let parts: Vec<&str> = name.splitn(2, "_").collect();

    if parts.len() < 2 {
        let error_message =
            "Formato inválido. Asegúrate de incluir al menos un guion bajo.".to_string();
        return Err(error_message);
    }

    // Formateo del prefijo
    let prefix = parts[0].to_uppercase(); // Convierte el prefijo a mayúsculas

    // Formateo del resto del texto
    let rest = parts[1]
        .replace("_", ": ") // Reemplaza guiones bajos por ": "
        .replace("-", " "); // Reemplaza guiones medios por espacios

    // Combina el prefijo formateado con el resto del texto
    let formatted_name = format!("{}: {}", prefix, rest);
    Ok(formatted_name)
}

pub fn sanitize_filename(s: &str) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Lógica específica para Windows
        let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        let sanitized = s
            .chars()
            .filter(|c| !invalid_chars.contains(c))
            .collect::<String>();
        Ok(sanitized)
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Lógica para otros sistemas (Linux, macOS, etc.)
        let invalid_chars = ['/']; // Ejemplo básico
        let sanitized = s
            .chars()
            .filter(|c| !invalid_chars.contains(c))
            .collect::<String>();
        Ok(sanitized)
    }
}

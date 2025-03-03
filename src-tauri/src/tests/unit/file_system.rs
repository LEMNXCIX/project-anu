#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use serde_json::json;

    // Mock para simular `get_item`
    fn mock_get_item(key: String) -> ApiResponse {
        if key == "ruta_base" {
            ApiResponse::new_success(
                json!("/mocked/downloads"),
                "Ruta base mockeada".to_string(),
            )
        } else {
            ApiResponse::new_error("Clave no encontrada".to_string(), vec![])
        }
    }

    #[test]
    fn test_list_directory_non_existent() {
        let response = list_directory("/non/existent/path");
        assert!(!response.is_success());
        assert_eq!(response.get_message(), "El directorio no existe: /non/existent/path");
    }

    #[test]
    fn test_list_directory_not_a_dir() {
        // Simulamos un archivo en lugar de un directorio
        let path = "/mocked/file.txt";
        let response = list_directory(path);
        assert!(!response.is_success());
        assert_eq!(response.get_message(), format!("El directorio no existe: {}", path));
    }

    #[test]
    fn test_create_directory_formatting_error() {
        // Simulamos un error en format_name_project
        let response = create_directory(""); // Suponiendo que "" causa error en format_name_project
        assert!(!response.is_success());
        assert!(response.get_message().contains("Error al formatear nombre"));
    }

    #[test]
    fn test_create_directory_already_exists() {
        unsafe {
            crate::service::config_user::get_item = mock_get_item;
        }
        let path = "/mocked/downloads/existing";
        let mock_path = Path::new(path);
        // Simulamos que el directorio ya existe
        let response = create_directory("existing");
        assert!(response.is_warning());
        assert_eq!(response.get_message(), "Ya existe un directorio creado para el proyecto.");
    }

    #[test]
    fn test_write_file_creation_failure() {
        let response = write_file("/non/writable/path.txt", "content");
        assert!(!response.is_success());
        assert_eq!(response.get_message(), "Error al abrir archivo para escritura");
    }

    #[test]
    fn test_list_directory_by_project_name_non_existent() {
        unsafe {
            crate::service::config_user::get_item = mock_get_item;
        }
        let response = list_directory_by_proyect_name("NonExistent");
        assert!(!response.is_success());
        assert_eq!(response.get_message(), "El directorio no existe");
    }
}
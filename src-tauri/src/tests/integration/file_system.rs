// tests/integration_tests.rs
use my_crate_name::list_directory_by_proyect_name; // Cambia "my_crate_name" por el nombre de tu crate
use my_crate_name::{list_directory, create_directory, write_file, ApiResponse, DirectoryListing};
use std::fs::{self, File};
use std::io::Read;
use tempfile::TempDir;
use serde_json::from_value;

#[test]
fn test_list_directory_integration() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    // Crear contenido en el directorio
    fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    File::create(temp_dir.path().join("file.txt")).unwrap();

    let response = list_directory(dir_path);
    assert!(response.is_success());

    let listing: DirectoryListing = from_value(response.get_data()).unwrap();
    assert_eq!(listing.path, dir_path);
    assert_eq!(listing.entries.len(), 2);
    assert!(listing.entries.iter().any(|e| e.name == "subdir" && e.is_directory));
    assert!(listing.entries.iter().any(|e| e.name == "file.txt" && !e.is_directory));
}

#[test]
fn test_create_and_list_directory_integration() {
    let temp_dir = TempDir::new().unwrap();
    unsafe {
        crate::service::config_user::get_item = |_| {
            ApiResponse::new_success(
                serde_json::json!(temp_dir.path().to_str().unwrap()),
                "Mocked ruta_base".to_string(),
            )
        };
    }

    // Crear un directorio
    let response_create = create_directory("TestProject");
    assert!(response_create.is_success());
    let created_path: String = from_value(response_create.get_data()).unwrap();
    assert!(std::path::Path::new(&created_path).exists());

    // Listar el directorio creado
    let response_list = list_directory(&created_path);
    assert!(response_list.is_success());
    let listing: DirectoryListing = from_value(response_list.get_data()).unwrap();
    assert_eq!(listing.path, created_path);
    assert_eq!(listing.entries.len(), 0); // Recién creado, vacío
}

#[test]
fn test_write_file_and_list_integration() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let content = "Integration test content";

    // Escribir un archivo
    let response_write = write_file(file_path.to_str().unwrap(), content);
    assert!(response_write.is_success());

    // Verificar el contenido
    let mut file_content = String::new();
    File::open(&file_path).unwrap().read_to_string(&mut file_content).unwrap();
    assert_eq!(file_content, content);

    // Listar el directorio
    let response_list = list_directory(temp_dir.path().to_str().unwrap());
    assert!(response_list.is_success());
    let listing: DirectoryListing = from_value(response_list.get_data()).unwrap();
    assert!(listing.entries.iter().any(|e| e.name == "test.txt" && !e.is_directory));
}

#[test]
fn test_list_directory_by_project_name_integration() {
    let temp_dir = TempDir::new().unwrap();
    unsafe {
        crate::service::config_user::get_item = |_| {
            ApiResponse::new_success(
                serde_json::json!(temp_dir.path().to_str().unwrap()),
                "Mocked ruta_base".to_string(),
            )
        };
    }

    // Crear un directorio de proyecto y un archivo dentro
    let project_name = "MyProject";
    let response_create = create_directory(project_name);
    assert!(response_create.is_success());
    let project_path: String = from_value(response_create.get_data()).unwrap();

    let file_path = std::path::Path::new(&project_path).join("file.txt");
    File::create(&file_path).unwrap();

    // Listar por nombre de proyecto
    let response_list = list_directory_by_proyect_name(project_name);
    assert!(response_list.is_success());
    let listing: DirectoryListing = from_value(response_list.get_data()).unwrap();
    assert_eq!(listing.entries.len(), 1);
    assert_eq!(listing.entries[0].name, "file.txt");
}
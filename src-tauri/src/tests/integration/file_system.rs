// tests/file_system_tests.rs
use crate::common::response::ApiResponse;
use crate::service::config_user::{ConfigProvider, get_item};
use crate::service::file_system::{create_directory, list_directory, list_directory_by_proyect_name, write_file};
use crate::service::models::file_system::DirectoryListing;
use std::fs::{self, File};
use std::io::Read;
use mockall::mock;
use tempfile::TempDir;
use serde_json::from_value;

mock! {
    pub ConfigUserMock {
        fn get_item(&self, key: String) -> ApiResponse;
    }
}

impl ConfigProvider for MockConfigUserMock {
    fn get_item(&self, key: String) -> ApiResponse {
        self.get_item(key)
    }
}

// Macro para ejecutar un bloque con un mock y limpiarlo automáticamente
macro_rules! with_mock {
    ($mock:expr, $body:expr) => {{
        let mock_static: &'static mut MockConfigUserMock = Box::leak(Box::new($mock));
        unsafe {
            crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
        }
        let result = $body;
        unsafe {
            crate::service::config_user::CONFIG_PROVIDER = None;
        }
        result
    }};
}

#[test]
fn test_list_directory_integration() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    // Crear contenido en el directorio
    fs::create_dir(temp_dir.path().join("subdir")).unwrap();
    File::create(temp_dir.path().join("file.txt")).unwrap();

    let response = list_directory(dir_path);
    assert!(response.is_success(), "Expected success, got {:?}", response);

    let listing: DirectoryListing = from_value(response.get_data().clone()).unwrap();
    assert_eq!(listing.path, dir_path);
    assert_eq!(listing.entries.len(), 2);
    assert!(listing.entries.iter().any(|e| e.name == "subdir" && e.is_directory));
    assert!(listing.entries.iter().any(|e| e.name == "file.txt" && !e.is_directory));
}

#[test]
fn test_create_and_list_directory_integration() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().to_str().unwrap().to_string();

    // Configurar el mock
    let mut mock = MockConfigUserMock::new();
    mock.expect_get_item()
        .withf(|key: &String| key == "ruta_base")
        .returning({
            let base_path = base_path.clone();
            move |_: String| ApiResponse::new_success(serde_json::json!(base_path.clone()), "".to_string())
        });

    with_mock!(mock, {
        // Crear un directorio
        let response_create = create_directory("TestProject");
        assert!(response_create.is_success(), "Expected success, got {:?}", response_create);
        let created_path: String = response_create.get_data().as_str().unwrap().to_string();
        assert!(std::path::Path::new(&created_path).exists());

        // Listar el directorio creado
        let response_list = list_directory(&created_path);
        assert!(response_list.is_success(), "Expected success, got {:?}", response_list);
        let listing: DirectoryListing = from_value(response_list.get_data().clone()).unwrap();
        assert_eq!(listing.path, created_path);
        assert_eq!(listing.entries.len(), 0); // Recién creado, vacío
    });
}

#[test]
fn test_list_directory_by_project_name_integration() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().to_str().unwrap().to_string();

    // Configurar el mock para que get_item devuelva la ruta temporal
    let mut mock = MockConfigUserMock::new();
    mock.expect_get_item()
        .withf(|key: &String| key == "ruta_base")
        .times(3) // Esperamos dos llamadas: una para create_directory y otra para list_directory_by_proyect_name
        .returning({
            let base_path = base_path.clone();
            move |_: String| {
                println!("get_item called, returning: {}", base_path); // Log para depuración
                ApiResponse::new_success(serde_json::json!(base_path.clone()), "".to_string())
            }
        });

    let mock_static: &'static mut MockConfigUserMock = Box::leak(Box::new(mock));
    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
    }

    // Crear un directorio de proyecto y un archivo dentro
    let project_name = "MyProjectByName";
    let response_create = create_directory(project_name);
    assert!(response_create.is_success(), "create_directory Expected success, got {:?}", response_create);
    let project_path: String = from_value(response_create.get_data().clone()).unwrap();

    let file_path = std::path::Path::new(&project_path).join("file.txt");
    File::create(&file_path).unwrap();

    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
    }

    // Listar por nombre de proyecto
    let response_list = list_directory_by_proyect_name(project_name);
    assert!(response_list.is_success(), "list_directory_by_proyect_name Expected success, got {:?}", response_list);
    let listing: DirectoryListing = from_value(response_list.get_data().clone()).unwrap();
    assert_eq!(listing.entries.len(), 1);
    assert_eq!(listing.entries[0].name, "file.txt");

    // Limpiar el mock
    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = None;
    }
}
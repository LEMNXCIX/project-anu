use crate::{
    common::response::ApiResponse,
    service::{
        config_user::ConfigProvider,
        file_system::{create_directory, list_directory_by_proyect_name},
        models::DirectoryListing,
    },
};
// tests/file_system_tests.rs
#[cfg(test)]
use crate::service::config_user::get_item;

use mockall::mock;
use std::{
    fs::{self, File},
    path::Path,
};
use tempfile::TempDir;

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

#[test]
fn test_create_directory_already_exists() {
    let tmp_dir = TempDir::new().unwrap();
    let dir_path = tmp_dir.path().join("rutatestexists");

    // Crear el directorio antes de la prueba
    fs::create_dir(&dir_path).expect("No se pudo crear el directorio de prueba");

    // Configurar el mock para devolver la ruta base correcta
    let mut mock = MockConfigUserMock::new();
    mock.expect_get_item()
        .withf(|key: &String| key == "ruta_base")
        .returning(move |_: String| {
            let base_path = tmp_dir.path().to_str().unwrap().to_string();
            log::info!("Returning base path: {}", base_path);
            ApiResponse::new_success(serde_json::json!(base_path), "".to_string())
        });

    let mock_static: &'static mut MockConfigUserMock = Box::leak(Box::new(mock));
    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
    }

    let result = create_directory("rutatestexists");
    assert!(result.is_warning(), "Expected warning, got {:?}", result);
    assert_eq!(
        result.get_message(),
        "Ya existe un directorio creado para el proyecto."
    );

    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = None;
    }
}

#[test]
fn test_create_directory_success() {
    let tmp_dir = TempDir::new().unwrap();
    let base_path = tmp_dir.path().to_str().unwrap().to_string();

    let mut mock = MockConfigUserMock::new();
    mock.expect_get_item()
        .withf(|key: &String| key == "ruta_base")
        .returning({
            let base_path = base_path.clone();
            move |_: String| ApiResponse::new_success(serde_json::json!(base_path), "".to_string())
        });

    let mock_static: &'static mut MockConfigUserMock = Box::leak(Box::new(mock));
    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
    }

    let result = create_directory("test_create_directory_success");
    assert!(result.is_success(), "Expected success, got {:?}", result); 
    let created_path = result.get_data().as_str().unwrap();
    assert!(Path::new(created_path).exists());
    assert!(Path::new(created_path).is_dir());

    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = None;
    }
}
#[test]
fn test_list_directory_by_project_name_success() {
    let tmp_dir = TempDir::new().unwrap();
    let base_path = tmp_dir.path().to_str().unwrap().to_string();
    let project_name = "rutatestsuccess";
    let project_path = tmp_dir.path().join(project_name);

    fs::create_dir(&project_path).unwrap();
    File::create(project_path.join("file.txt")).unwrap();

    let mut mock = MockConfigUserMock::new();
    mock.expect_get_item()
        .withf(|key: &String| key == "ruta_base")
        .returning(move |_: String| {
            ApiResponse::new_success(serde_json::json!(base_path.to_string()), "".to_string())
        });

    let mock_static: &'static mut MockConfigUserMock = Box::leak(Box::new(mock));
    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = Some(mock_static);
    }

    let result = list_directory_by_proyect_name(project_name);
    assert!(result.is_success(), "Expected warning, got {:?}", result);
    let data = result.get_data();
    let listing: DirectoryListing = serde_json::from_value(data.clone()).unwrap();
    assert_eq!(listing.entries.len(), 1);
    assert_eq!(listing.entries[0].name, "file.txt");

    unsafe {
        crate::service::config_user::CONFIG_PROVIDER = None;
    }
}

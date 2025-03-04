use serde_json::json;
use crate::common::response::ApiResponse;

#[tokio::test] // Usamos tokio::test por consistencia con otros ejemplos, aunque aquí no es estrictamente necesario
async fn test_new_default() {
    let response = ApiResponse::new();
    assert!(!response.is_success());
    assert!(!response.is_error());
    assert_eq!(response.get_message(), "");
    assert_eq!(response.get_data(), &serde_json::Value::Null);
    assert!(response.get_error_details().is_empty());
}

#[tokio::test]
async fn test_new_success_valid() {
    let data = json!({"id": 1});
    let message = "Operación exitosa".to_string();
    let response = ApiResponse::new_success(data.clone(), message.clone());
    assert!(response.is_success());
    assert!(!response.is_error());
    assert_eq!(response.get_message(), message);
    assert_eq!(response.get_data(), &data);
    assert!(response.get_error_details().is_empty());
}

#[tokio::test]
async fn test_new_warning_valid() {
    let message = "Advertencia generada".to_string();
    let details = vec!["Detalle 1".to_string()];
    let response = ApiResponse::new_warning(message.clone(), details.clone());
    assert!(!response.is_success());
    assert!(!response.is_error());
    assert_eq!(response.get_message(), message);
    assert_eq!(response.get_data(), &serde_json::Value::Null);
    assert_eq!(response.get_error_details(), &details);
}

#[tokio::test]
async fn test_new_error_valid() {
    let message = "Error ocurrido".to_string();
    let details = vec!["Detalle 1".to_string(), "Detalle 2".to_string()];
    let response = ApiResponse::new_error(message.clone(), details.clone());
    assert!(!response.is_success());
    assert!(response.is_error());
    assert_eq!(response.get_message(), message);
    assert_eq!(response.get_data(), &serde_json::Value::Null);
    assert_eq!(response.get_error_details(), &details);
}

#[tokio::test]
async fn test_add_error_detail_success() {
    let mut response = ApiResponse::new_error("Error".to_string(), vec![]);
    response.add_error_detail("Nuevo detalle".to_string());
    assert_eq!(response.get_error_details(), &["Nuevo detalle".to_string()]);
}

#[tokio::test]
async fn test_add_error_detail_on_success_fails() {
    let mut response = ApiResponse::new_success(json!({"id": 1}), "Éxito".to_string());
    response.add_error_detail("Intento fallido".to_string());
    assert!(response.get_error_details().is_empty());
}

#[tokio::test]
async fn test_set_data_success() {
    let mut response = ApiResponse::new_success(serde_json::Value::Null, "Éxito".to_string());
    let new_data = json!({"id": 2});
    response.set_data(new_data.clone());
    assert_eq!(response.get_data(), &new_data);
}

#[tokio::test]
async fn test_set_data_on_error_fails() {
    let mut response = ApiResponse::new_error("Error".to_string(), vec![]);
    let new_data = json!({"id": 2});
    response.set_data(new_data);
    assert_eq!(response.get_data(), &serde_json::Value::Null);
}

#[tokio::test]
async fn test_is_success_and_is_error_mutually_exclusive() {
    let success_response = ApiResponse::new_success(json!({"id": 1}), "Éxito".to_string());
    assert!(success_response.is_success());
    assert!(!success_response.is_error());

    let error_response = ApiResponse::new_error("Error".to_string(), vec![]);
    assert!(!error_response.is_success());
    assert!(error_response.is_error());

    let warning_response = ApiResponse::new_warning("Advertencia".to_string(), vec![]);
    assert!(!warning_response.is_success());
    assert!(!warning_response.is_error());
}
use crate::common::response::ApiResponse;
use serde_json::json;

#[test]
fn test_new_response() {
    let response = ApiResponse::new();
    assert!(!response.is_success());
    assert_eq!(response.data, json!(null));
    assert!(response.get_message().is_empty());
    assert!(response.error_details.is_empty());
}

#[test]
fn test_new_success() {
    let response = ApiResponse::new_success(json!({"id": 1}), "Éxito".to_string());
    assert!(response.is_success());
    assert_eq!(response.data, json!({"id": 1}));
    assert_eq!(response.get_message(), "Éxito");
    assert!(response.error_details.is_empty());
}

#[test]
fn test_new_error() {
    let response = ApiResponse::new_error(
        "Error".to_string(),
        vec!["Detalle 1".to_string(), "Detalle 2".to_string()],
    );
    assert!(!response.is_success());
    assert_eq!(response.data, json!(null));
    assert_eq!(response.get_message(), "Error");
    assert_eq!(response.error_details, vec!["Detalle 1", "Detalle 2"]);
}

#[test]
fn test_add_error_detail() {
    let mut response = ApiResponse::new_error("Error".to_string(), vec![]);
    response.add_error_detail("Nuevo detalle".to_string());
    assert_eq!(response.error_details, vec!["Nuevo detalle"]);
}

#[test]
fn test_set_data() {
    let mut response = ApiResponse::new_success(json!(null), "Éxito".to_string());
    response.set_data(json!({"id": 2}));
    assert_eq!(response.data, json!({"id": 2}));
}
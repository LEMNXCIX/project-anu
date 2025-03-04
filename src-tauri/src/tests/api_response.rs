use crate::common::response::ApiResponse;
use serde_json::json;

#[test]
fn test_new() {
    let response = ApiResponse::new();
    assert!(!response.success);
    assert!(!response.error);
    assert_eq!(response.data, serde_json::Value::Null);
    assert!(response.message.is_empty());
    assert!(response.error_details.is_empty());
}

#[test]
fn test_new_success() {
    let data = json!({"key": "value"});
    let message = String::from("Success message");
    let response = ApiResponse::new_success(data.clone(), message.clone());
    assert!(response.success);
    assert!(!response.error);
    assert_eq!(response.data, data);
    assert_eq!(response.message, message);
    assert!(response.error_details.is_empty());
}

#[test]
fn test_new_error() {
    let message = String::from("Error message");
    let details = vec![String::from("Detail 1"), String::from("Detail 2")];
    let response = ApiResponse::new_error(message.clone(), details.clone());
    assert!(!response.success);
    assert!(response.error);
    assert_eq!(response.data, serde_json::Value::Null);
    assert_eq!(response.message, message);
    assert_eq!(response.error_details, details);
}

#[test]
fn test_add_error_detail() {
    let mut response = ApiResponse::new_error(String::from("Error message"), Vec::new());
    response.add_error_detail(String::from("Detail 1"));
    assert_eq!(response.error_details.len(), 1);
    assert_eq!(response.error_details[0], "Detail 1");
}

#[test]
fn test_set_data() {
    let mut response = ApiResponse::new_success(serde_json::Value::Null, String::from("Success message"));
    let data = json!({"key": "value"});
    response.set_data(data.clone());
    assert_eq!(response.data, data);
}

#[test]
fn test_is_success() {
    let response = ApiResponse::new_success(serde_json::Value::Null, String::from("Success message"));
    assert!(response.is_success());
}

#[test]
fn test_is_error() {
    let response = ApiResponse::new_error(String::from("Error message"), Vec::new());
    assert!(response.is_error());
}

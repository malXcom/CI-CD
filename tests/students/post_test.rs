mod common {
    include!("../common.rs");
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn test_create_valid_returns_201_with_id() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/students")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "firstName": "Marie",
                "lastName": "Curie",
                "email": "marie@example.com",
                "grade": 19.5,
                "field": "physique"
            }).to_string())).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    assert!(body["id"].as_u64().is_some());
    assert_eq!(body["firstName"], "Marie");
    assert_eq!(body["email"], "marie@example.com");
}

#[tokio::test]
async fn test_create_missing_field_returns_400() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/students")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "firstName": "Marie",
                "lastName": "Curie"
                // missing email, grade, field
            }).to_string())).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_invalid_grade_returns_422() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/students")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "firstName": "Test",
                "lastName": "User",
                "email": "test@example.com",
                "grade": 25.0,
                "field": "chimie"
            }).to_string())).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_create_duplicate_email_returns_409() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("POST")
            .uri("/students")
            .header("content-type", "application/json")
            .body(Body::from(json!({
                "firstName": "John",
                "lastName": "Doe",
                "email": "john@example.com",  // already in store
                "grade": 10.0,
                "field": "chimie"
            }).to_string())).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

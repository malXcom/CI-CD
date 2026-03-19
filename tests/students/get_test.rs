mod common {
    include!("../common.rs");
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_all_returns_200_and_array() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    assert!(body.is_array());
}

#[tokio::test]
async fn test_get_all_returns_initial_students() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    let students = body.as_array().unwrap();
    assert_eq!(students.len(), 2);
    assert_eq!(students[0]["firstName"], "John");
    assert_eq!(students[1]["firstName"], "Bob");
}

#[tokio::test]
async fn test_get_by_id_returns_correct_student() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students/1")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    assert_eq!(body["id"], 1);
    assert_eq!(body["firstName"], "John");
    assert_eq!(body["email"], "john@example.com");
}


#[tokio::test]
async fn test_get_by_id_not_found_returns_404() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students/999")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    assert!(body["error"].is_string());
}


#[tokio::test]
async fn test_get_by_invalid_id_returns_400() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students/abc")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}


#[tokio::test]
async fn test_stats_returns_expected_fields() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students/stats")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    assert!(body["totalStudents"].as_i64().is_some());
    assert!(body["averageGrade"].as_f64().is_some());
    assert!(body["studentsByField"].is_object());
    assert!(body["bestStudent"].is_object());
    assert!(body["bestStudent"]["id"].as_u64().is_some());
}

#[tokio::test]
async fn test_search_returns_matching_students() {
    let resp = common::app()
        .oneshot(Request::builder()
            .uri("/students/search?q=john")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = serde_json::from_slice(
        &resp.into_body().collect().await.unwrap().to_bytes()
    ).unwrap();

    let results = body.as_array().unwrap();
    assert!(!results.is_empty());
    assert_eq!(results[0]["firstName"], "John");
}

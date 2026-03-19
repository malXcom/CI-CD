mod common {
    include!("../common.rs");
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

#[tokio::test]
async fn test_update_valid_returns_200_with_updated_student() {
    let resp = common::app()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/students/1")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "grade": 18.0
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value =
        serde_json::from_slice(&resp.into_body().collect().await.unwrap().to_bytes()).unwrap();

    assert_eq!(body["id"], 1);
    assert_eq!(body["grade"], 18.0);
    assert_eq!(body["firstName"], "John");
}

#[tokio::test]
async fn test_update_nonexistent_returns_404() {
    let resp = common::app()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/students/999")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "grade": 15.0
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

mod common {
    include!("../common.rs");
}

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_delete_valid_returns_204() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("DELETE")
            .uri("/students/1")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_delete_nonexistent_returns_404() {
    let resp = common::app()
        .oneshot(Request::builder()
            .method("DELETE")
            .uri("/students/999")
            .body(Body::empty()).unwrap())
        .await.unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

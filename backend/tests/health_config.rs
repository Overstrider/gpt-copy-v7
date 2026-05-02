mod support;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use gpt_copy_v7_backend::{config::AppConfig, routes::health::HealthResponse};
use tower::ServiceExt;

use support::{MockChatProvider, app_with_provider, test_pool};

#[test]
fn config_defaults_openrouter_model_when_unset() {
    let config = AppConfig::from_env_source(|_| None).expect("load defaults");

    assert_eq!(config.openrouter.api_key, None);
    assert_eq!(
        config.openrouter.model,
        "nvidia/nemotron-3-super-120b-a12b:free"
    );
    assert_eq!(config.frontend_origin, "http://localhost:3000");
}

#[tokio::test]
async fn health_route_returns_status_without_provider_key() {
    let app = app_with_provider(test_pool().await, MockChatProvider::default());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let health: HealthResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(health.status, "ok");
    assert_eq!(health.service, "gpt-copy-v7-backend");
}

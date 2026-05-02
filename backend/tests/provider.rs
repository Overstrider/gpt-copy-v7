use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    Json, Router,
    body::{Body, Bytes},
    http::{StatusCode, header},
    response::Response,
    routing::post,
};
use futures::{StreamExt, stream};
use gpt_copy_v7_backend::{
    config::OpenRouterConfig,
    provider::{ChatProvider, OpenRouterProvider, ProviderError, ProviderMessage},
};
use serde_json::{Value, json};
use tokio::net::TcpListener;

fn provider_config(base_url: String) -> OpenRouterConfig {
    OpenRouterConfig {
        api_key: Some("test-key".into()),
        model: "test-model".into(),
        base_url,
        timeout_secs: 30,
    }
}

async fn spawn_fake_provider(router: Router) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    format!("http://{address}")
}

#[tokio::test]
async fn openrouter_provider_maps_missing_key_rate_limit_timeout_and_invalid_payload() {
    let missing_key = OpenRouterProvider::new(OpenRouterConfig {
        api_key: None,
        model: "test-model".into(),
        base_url: "http://127.0.0.1:1".into(),
        timeout_secs: 30,
    });
    let err = missing_key.complete(vec![]).await.unwrap_err();
    assert!(matches!(err, ProviderError::NotConfigured));

    let rate_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(|| async { StatusCode::TOO_MANY_REQUESTS }),
    ))
    .await;
    let err = OpenRouterProvider::new(provider_config(rate_url))
        .complete(vec![])
        .await
        .unwrap_err();
    assert!(matches!(err, ProviderError::RateLimited));

    let invalid_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(|| async { Json(json!({ "choices": [] })) }),
    ))
    .await;
    let err = OpenRouterProvider::new(provider_config(invalid_url))
        .complete(vec![])
        .await
        .unwrap_err();
    assert!(matches!(err, ProviderError::InvalidResponse(_)));

    let slow_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(|| async {
            tokio::time::sleep(Duration::from_millis(200)).await;
            Json(json!({ "choices": [{ "message": { "content": "late" } }] }))
        }),
    ))
    .await;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(20))
        .build()
        .unwrap();
    let err = OpenRouterProvider::with_client(provider_config(slow_url), client)
        .complete(vec![])
        .await
        .unwrap_err();
    assert!(matches!(err, ProviderError::Timeout));
}

#[tokio::test]
async fn openrouter_provider_parses_successful_completion_and_stream_errors() {
    let success_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(|Json(_body): Json<Value>| async {
            Json(json!({ "choices": [{ "message": { "content": "assistant reply" } }] }))
        }),
    ))
    .await;
    let provider = OpenRouterProvider::new(provider_config(success_url));
    let content = provider
        .complete(vec![ProviderMessage::user("hello")])
        .await
        .unwrap();
    assert_eq!(content, "assistant reply");

    let invalid_stream_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(|| async { "data: {not-json}\n\n" }),
    ))
    .await;
    let mut stream = OpenRouterProvider::new(provider_config(invalid_stream_url))
        .stream(vec![ProviderMessage::user("hello")])
        .await
        .unwrap();
    let first = stream.next().await.unwrap().unwrap_err();
    assert!(matches!(first, ProviderError::InvalidResponse(_)));
}

#[tokio::test]
async fn openrouter_provider_buffers_utf8_split_across_stream_chunks() {
    let frame = "data: {\"choices\":[{\"delta\":{\"content\":\"olá\"}}]}\n\n";
    let bytes = frame.as_bytes();
    let split_at = bytes
        .iter()
        .position(|byte| *byte == 0xc3)
        .map(|index| index + 1)
        .unwrap();
    let chunks = Arc::new(vec![
        bytes[..split_at].to_vec(),
        bytes[split_at..].to_vec(),
        b"data: [DONE]\n\n".to_vec(),
    ]);

    let stream_url = spawn_fake_provider(Router::new().route(
        "/chat/completions",
        post(move || {
            let chunks = Arc::clone(&chunks);
            async move {
                let chunks = (*chunks).clone();
                let body_stream = stream::iter(
                    chunks
                        .into_iter()
                        .map(|chunk| Ok::<Bytes, Infallible>(Bytes::from(chunk))),
                );
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/event-stream")
                    .body(Body::from_stream(body_stream))
                    .unwrap()
            }
        }),
    ))
    .await;

    let mut stream = OpenRouterProvider::new(provider_config(stream_url))
        .stream(vec![ProviderMessage::user("hello")])
        .await
        .unwrap();

    assert_eq!(stream.next().await.unwrap().unwrap(), "olá");
    assert!(stream.next().await.is_none());
}

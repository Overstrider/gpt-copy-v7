mod support;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use gpt_copy_v7_backend::{models::MessageRole, provider::ProviderError, repository};
use serde_json::json;
use tower::ServiceExt;

use support::{MockChatProvider, app_with_provider, test_pool};

#[tokio::test]
async fn chat_stream_success_emits_deltas_completion_and_persists_assistant_after_completion() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::default();
    provider.push_stream(Ok(vec![Ok("hello".into()), Ok(" world".into())]));
    let app = app_with_provider(pool.clone(), provider.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/conversations/{}/messages/stream",
                    conversation.id
                ))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hi" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("event: delta"));
    assert!(text.contains("hello"));
    assert!(text.contains("event: complete"));
    assert_eq!(provider.stream_calls(), 1);

    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, MessageRole::User);
    assert_eq!(messages[1].role, MessageRole::Assistant);
    assert_eq!(messages[1].content, "hello world");
}

#[tokio::test]
async fn chat_stream_interrupted_provider_emits_error_and_does_not_persist_partial_assistant() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::default();
    provider.push_stream(Ok(vec![
        Ok("partial".into()),
        Err(ProviderError::InterruptedStream),
    ]));
    let app = app_with_provider(pool.clone(), provider.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/conversations/{}/messages/stream",
                    conversation.id
                ))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hi" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();
    assert!(text.contains("event: delta"));
    assert!(text.contains("event: error"));
    assert!(text.contains("provider_stream_interrupted"));

    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, MessageRole::User);
}

#[tokio::test]
async fn chat_stream_initial_provider_errors_map_to_structured_json() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::default();
    provider.push_stream(Err(ProviderError::Timeout));
    let app = app_with_provider(pool, provider.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/conversations/{}/messages/stream",
                    conversation.id
                ))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hi" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["error"]["code"], "provider_timeout");
    assert_eq!(provider.stream_calls(), 1);
}

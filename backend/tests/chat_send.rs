mod support;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
    response::Response,
};
use gpt_copy_v7_backend::{models::MessageRole, provider::ProviderError, repository};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

use support::{MockChatProvider, app_with_provider, test_pool};

async fn json_response(response: Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn chat_send_persists_user_then_assistant_with_mocked_provider() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::with_completion(Ok("assistant reply".into()));
    let app = app_with_provider(pool.clone(), provider.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/conversations/{}/messages", conversation.id))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hello" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = json_response(response).await;
    assert_eq!(body["assistant_message"]["content"], "assistant reply");
    assert_eq!(provider.complete_calls(), 1);

    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, MessageRole::User);
    assert_eq!(messages[1].role, MessageRole::Assistant);
}

#[tokio::test]
async fn chat_send_provider_failure_keeps_user_message_without_assistant() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::with_completion(Err(ProviderError::RateLimited));
    let app = app_with_provider(pool.clone(), provider.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/conversations/{}/messages", conversation.id))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hello" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    let body = json_response(response).await;
    assert_eq!(body["error"]["code"], "provider_rate_limited");
    assert_eq!(provider.complete_calls(), 1);

    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, MessageRole::User);
}

#[tokio::test]
async fn chat_send_validation_and_missing_conversation_do_not_call_provider() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, None).await.unwrap();
    let provider = MockChatProvider::with_completion(Ok("should not be used".into()));
    let app = app_with_provider(pool.clone(), provider.clone());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/conversations/{}/messages", conversation.id))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "   " }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let missing_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/conversations/{missing_id}/messages"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "content": "hello" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    assert_eq!(provider.complete_calls(), 0);
    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert!(messages.is_empty());
}

mod support;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
    response::Response,
};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

use support::{MockChatProvider, app_with_provider, test_pool};

async fn json_response(response: Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn conversation_routes_create_list_and_return_messages() {
    let app = app_with_provider(test_pool().await, MockChatProvider::default());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/conversations")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({ "title": "Chat" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let created = json_response(response).await;
    let conversation_id = created["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/conversations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let list = json_response(response).await;
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["id"], conversation_id);

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/conversations/{conversation_id}/messages"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let messages = json_response(response).await;
    assert!(messages.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn conversation_routes_map_malformed_and_missing_message_list_ids() {
    let app = app_with_provider(test_pool().await, MockChatProvider::default());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/conversations/not-a-uuid/messages")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = json_response(response).await;
    assert_eq!(body["error"]["code"], "validation_error");

    let missing_id = Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/conversations/{missing_id}/messages"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = json_response(response).await;
    assert_eq!(body["error"]["code"], "conversation_not_found");
}

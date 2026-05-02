mod support;

use gpt_copy_v7_backend::{
    errors::AppError,
    models::MessageRole,
    repository,
    validation::{MAX_MESSAGE_CONTENT_LEN, validate_conversation_id, validate_message_content},
};
use uuid::Uuid;

use support::test_pool;

#[test]
fn validation_rejects_malformed_ids_and_invalid_content() {
    assert_eq!(
        validate_conversation_id("not-a-uuid").unwrap_err().code(),
        "validation_error"
    );
    assert_eq!(
        validate_message_content("").unwrap_err().code(),
        "validation_error"
    );
    assert_eq!(
        validate_message_content("   ").unwrap_err().code(),
        "validation_error"
    );
    assert_eq!(
        validate_message_content(&"x".repeat(MAX_MESSAGE_CONTENT_LEN + 1))
            .unwrap_err()
            .code(),
        "validation_error"
    );
}

#[tokio::test]
async fn repository_creates_lists_loads_conversations_and_ordered_messages() {
    let pool = test_pool().await;
    let conversation = repository::create_conversation(&pool, Some("First chat".into()))
        .await
        .unwrap();

    let loaded = repository::load_conversation(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(loaded.id, conversation.id);
    assert_eq!(loaded.title.as_deref(), Some("First chat"));

    let conversations = repository::list_conversations(&pool).await.unwrap();
    assert_eq!(conversations.len(), 1);
    assert_eq!(conversations[0].id, conversation.id);

    let user =
        repository::insert_message(&pool, conversation.id, MessageRole::User, "hello".into())
            .await
            .unwrap();
    let assistant = repository::insert_message(
        &pool,
        conversation.id,
        MessageRole::Assistant,
        "hi there".into(),
    )
    .await
    .unwrap();

    let messages = repository::list_messages(&pool, conversation.id)
        .await
        .unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].id, user.id);
    assert_eq!(messages[1].id, assistant.id);
}

#[tokio::test]
async fn repository_missing_conversation_maps_to_not_found() {
    let pool = test_pool().await;
    let err = repository::list_messages(&pool, Uuid::new_v4())
        .await
        .unwrap_err();

    assert!(matches!(err, AppError::ConversationNotFound));
    assert_eq!(err.code(), "conversation_not_found");
}

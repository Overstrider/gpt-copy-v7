use std::{convert::Infallible, pin::Pin};

use async_stream::stream;
use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    AppState,
    errors::{AppError, ErrorEnvelope},
    models::{Message, MessageRole},
    provider::{ProviderError, ProviderMessage},
    repository, validation,
};

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub user_message: Message,
    pub assistant_message: Message,
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<Vec<Message>>, AppError> {
    let conversation_id = validation::validate_conversation_id(&conversation_id)?;
    let messages = repository::list_messages(&state.pool, conversation_id).await?;
    Ok(Json(messages))
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    payload: Result<Json<SendMessageRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<SendMessageResponse>), AppError> {
    let Json(payload) = payload.map_err(|error| AppError::invalid_json(error.body_text()))?;
    let conversation_id = validation::validate_conversation_id(&conversation_id)?;
    let content = validation::validate_message_content(&payload.content)?;

    repository::load_conversation(&state.pool, conversation_id).await?;
    let user_message =
        repository::insert_message(&state.pool, conversation_id, MessageRole::User, content)
            .await?;
    let provider_messages = provider_messages_for_conversation(&state, conversation_id).await?;
    let assistant_content = state.provider.complete(provider_messages).await?;
    let assistant_message = repository::insert_message(
        &state.pool,
        conversation_id,
        MessageRole::Assistant,
        assistant_content,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(SendMessageResponse {
            user_message,
            assistant_message,
        }),
    ))
}

pub async fn stream_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    payload: Result<Json<SendMessageRequest>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(payload) = payload.map_err(|error| AppError::invalid_json(error.body_text()))?;
    let conversation_id = validation::validate_conversation_id(&conversation_id)?;
    let content = validation::validate_message_content(&payload.content)?;

    repository::load_conversation(&state.pool, conversation_id).await?;
    repository::insert_message(&state.pool, conversation_id, MessageRole::User, content).await?;
    let provider_messages = provider_messages_for_conversation(&state, conversation_id).await?;
    let mut provider_stream = state.provider.stream(provider_messages).await?;
    let pool = state.pool.clone();

    let events = stream! {
        let mut assistant_content = String::new();

        while let Some(chunk) = provider_stream.next().await {
            match chunk {
                Ok(delta) => {
                    assistant_content.push_str(&delta);
                    yield Ok(sse_event("delta", json!({ "content": delta })));
                }
                Err(error) => {
                    yield Ok(error_event(AppError::from(error)));
                    return;
                }
            }
        }

        if assistant_content.is_empty() {
            yield Ok(error_event(AppError::from(ProviderError::InvalidResponse(
                "empty streamed assistant content".to_owned(),
            ))));
            return;
        }

        match repository::insert_message(
            &pool,
            conversation_id,
            MessageRole::Assistant,
            assistant_content,
        )
        .await
        {
            Ok(message) => {
                yield Ok(sse_event("complete", json!({ "message": message })));
            }
            Err(error) => {
                yield Ok(error_event(error));
            }
        }
    };

    let events: Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>> = Box::pin(events);
    Ok(Sse::new(events).keep_alive(KeepAlive::default()))
}

async fn provider_messages_for_conversation(
    state: &AppState,
    conversation_id: uuid::Uuid,
) -> Result<Vec<ProviderMessage>, AppError> {
    let messages = repository::list_messages(&state.pool, conversation_id).await?;
    Ok(messages.iter().map(ProviderMessage::from_message).collect())
}

fn sse_event(payload_type: &'static str, payload: serde_json::Value) -> Event {
    Event::default()
        .event(payload_type)
        .data(payload.to_string())
}

fn error_event(error: AppError) -> Event {
    let envelope: ErrorEnvelope = error.envelope();
    Event::default()
        .event("error")
        .data(serde_json::to_string(&envelope).expect("error envelope serialization cannot fail"))
}

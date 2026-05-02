use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
};
use serde::Deserialize;

use crate::{AppState, errors::AppError, repository, validation};

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub title: Option<String>,
}

pub async fn list_conversations(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conversations = repository::list_conversations(&state.pool).await?;
    Ok(Json(
        serde_json::to_value(conversations).expect("conversation serialization cannot fail"),
    ))
}

pub async fn create_conversation(
    State(state): State<AppState>,
    payload: Result<Json<CreateConversationRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let Json(payload) = payload.map_err(|error| AppError::invalid_json(error.body_text()))?;
    let title = validation::validate_title(payload.title.as_deref())?;
    let conversation = repository::create_conversation(&state.pool, title).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::to_value(conversation).expect("conversation serialization cannot fail")),
    ))
}

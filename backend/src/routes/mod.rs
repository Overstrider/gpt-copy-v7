use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub mod health;

mod conversations;
mod messages;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route(
            "/api/conversations",
            get(conversations::list_conversations).post(conversations::create_conversation),
        )
        .route(
            "/api/conversations/{conversation_id}/messages",
            get(messages::list_messages).post(messages::send_message),
        )
        .route(
            "/api/conversations/{conversation_id}/messages/stream",
            post(messages::stream_message),
        )
        .with_state(state)
}

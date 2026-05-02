use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Value, json};

use crate::provider::ProviderError;

#[derive(Debug, Serialize)]
pub struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
    pub details: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{message}")]
    Validation {
        message: String,
        details: Option<Value>,
    },
    #[error("conversation not found")]
    ConversationNotFound,
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error("database operation failed")]
    Database,
}

impl AppError {
    pub fn validation(message: impl Into<String>, details: Option<Value>) -> Self {
        Self::Validation {
            message: message.into(),
            details,
        }
    }

    pub fn invalid_json(message: impl Into<String>) -> Self {
        Self::validation(
            "invalid JSON request body",
            Some(json!({ "reason": message.into() })),
        )
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation { .. } => "validation_error",
            Self::ConversationNotFound => "conversation_not_found",
            Self::Provider(error) => error.code(),
            Self::Database => "internal_error",
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::Validation { .. } => StatusCode::BAD_REQUEST,
            Self::ConversationNotFound => StatusCode::NOT_FOUND,
            Self::Provider(error) => error.status(),
            Self::Database => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn envelope(&self) -> ErrorEnvelope {
        ErrorEnvelope {
            error: ErrorBody {
                code: self.code(),
                message: self.public_message(),
                details: self.details(),
            },
        }
    }

    fn public_message(&self) -> String {
        match self {
            Self::Validation { message, .. } => message.clone(),
            Self::ConversationNotFound => "conversation not found".to_owned(),
            Self::Provider(error) => error.public_message().to_owned(),
            Self::Database => "internal server error".to_owned(),
        }
    }

    fn details(&self) -> Option<Value> {
        match self {
            Self::Validation { details, .. } => details.clone(),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!(%error, "database error");
        Self::Database
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        (status, Json(self.envelope())).into_response()
    }
}

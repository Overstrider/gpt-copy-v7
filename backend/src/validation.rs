use serde_json::json;
use uuid::Uuid;

use crate::errors::AppError;

pub const MAX_MESSAGE_CONTENT_LEN: usize = 32_000;
pub const MAX_TITLE_LEN: usize = 120;

pub fn validate_conversation_id(value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value).map_err(|_| {
        AppError::validation(
            "invalid conversation_id",
            Some(json!({
                "field": "conversation_id",
                "reason": "malformed_id"
            })),
        )
    })
}

pub fn validate_message_content(value: &str) -> Result<String, AppError> {
    if value.trim().is_empty() {
        return Err(AppError::validation(
            "message content is required",
            Some(json!({
                "field": "content",
                "reason": "empty"
            })),
        ));
    }

    if value.len() > MAX_MESSAGE_CONTENT_LEN {
        return Err(AppError::validation(
            "message content is too long",
            Some(json!({
                "field": "content",
                "reason": "too_large",
                "max": MAX_MESSAGE_CONTENT_LEN
            })),
        ));
    }

    Ok(value.to_owned())
}

pub fn validate_title(value: Option<&str>) -> Result<Option<String>, AppError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let title = value.trim();
    if title.is_empty() {
        return Ok(None);
    }
    if title.len() > MAX_TITLE_LEN {
        return Err(AppError::validation(
            "conversation title is too long",
            Some(json!({
                "field": "title",
                "reason": "too_large",
                "max": MAX_TITLE_LEN
            })),
        ));
    }
    Ok(Some(title.to_owned()))
}

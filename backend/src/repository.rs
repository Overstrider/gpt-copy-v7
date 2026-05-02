use chrono::{DateTime, SecondsFormat, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{Conversation, Message, MessageRole},
};

pub async fn create_conversation(
    pool: &SqlitePool,
    title: Option<String>,
) -> Result<Conversation, AppError> {
    let id = Uuid::new_v4();
    let now = now_text();

    sqlx::query(
        r#"
        INSERT INTO conversations (id, title, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
    )
    .bind(id.to_string())
    .bind(title)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    load_conversation(pool, id).await
}

pub async fn list_conversations(pool: &SqlitePool) -> Result<Vec<Conversation>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, title, created_at, updated_at
        FROM conversations
        ORDER BY updated_at DESC, created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(conversation_from_row).collect()
}

pub async fn load_conversation(
    pool: &SqlitePool,
    conversation_id: Uuid,
) -> Result<Conversation, AppError> {
    let row = sqlx::query(
        r#"
        SELECT id, title, created_at, updated_at
        FROM conversations
        WHERE id = ?1
        "#,
    )
    .bind(conversation_id.to_string())
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::ConversationNotFound)?;

    conversation_from_row(row)
}

pub async fn insert_message(
    pool: &SqlitePool,
    conversation_id: Uuid,
    role: MessageRole,
    content: String,
) -> Result<Message, AppError> {
    load_conversation(pool, conversation_id).await?;

    let id = Uuid::new_v4();
    let now = now_text();

    sqlx::query(
        r#"
        INSERT INTO messages (id, conversation_id, role, content, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(id.to_string())
    .bind(conversation_id.to_string())
    .bind(role.as_str())
    .bind(content)
    .bind(&now)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        UPDATE conversations
        SET updated_at = ?1
        WHERE id = ?2
        "#,
    )
    .bind(&now)
    .bind(conversation_id.to_string())
    .execute(pool)
    .await?;

    load_message(pool, id).await
}

pub async fn list_messages(
    pool: &SqlitePool,
    conversation_id: Uuid,
) -> Result<Vec<Message>, AppError> {
    load_conversation(pool, conversation_id).await?;

    let rows = sqlx::query(
        r#"
        SELECT id, conversation_id, role, content, created_at
        FROM messages
        WHERE conversation_id = ?1
        ORDER BY created_at ASC, rowid ASC
        "#,
    )
    .bind(conversation_id.to_string())
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(message_from_row).collect()
}

async fn load_message(pool: &SqlitePool, message_id: Uuid) -> Result<Message, AppError> {
    let row = sqlx::query(
        r#"
        SELECT id, conversation_id, role, content, created_at
        FROM messages
        WHERE id = ?1
        "#,
    )
    .bind(message_id.to_string())
    .fetch_one(pool)
    .await?;

    message_from_row(row)
}

fn conversation_from_row(row: sqlx::sqlite::SqliteRow) -> Result<Conversation, AppError> {
    Ok(Conversation {
        id: parse_uuid(row.get::<String, _>("id").as_str())?,
        title: row.get("title"),
        created_at: parse_timestamp(row.get::<String, _>("created_at").as_str())?,
        updated_at: parse_timestamp(row.get::<String, _>("updated_at").as_str())?,
    })
}

fn message_from_row(row: sqlx::sqlite::SqliteRow) -> Result<Message, AppError> {
    let role = row.get::<String, _>("role");
    Ok(Message {
        id: parse_uuid(row.get::<String, _>("id").as_str())?,
        conversation_id: parse_uuid(row.get::<String, _>("conversation_id").as_str())?,
        role: MessageRole::from_db(&role).ok_or(AppError::Database)?,
        content: row.get("content"),
        created_at: parse_timestamp(row.get::<String, _>("created_at").as_str())?,
    })
}

fn parse_uuid(value: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(value).map_err(|error| {
        tracing::error!(%error, "invalid uuid persisted in database");
        AppError::Database
    })
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| {
            tracing::error!(%error, "invalid timestamp persisted in database");
            AppError::Database
        })
}

fn now_text() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true)
}

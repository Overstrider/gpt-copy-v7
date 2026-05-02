#![allow(dead_code)]

use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use async_trait::async_trait;
use axum::Router;
use futures::stream;
use gpt_copy_v7_backend::{
    AppState, build_router, db,
    provider::{ChatProvider, ProviderError, ProviderMessage, ProviderStream},
};
use sqlx::SqlitePool;

type ScriptedStream = Result<Vec<Result<String, ProviderError>>, ProviderError>;

#[derive(Clone, Default)]
pub struct MockChatProvider {
    completions: Arc<Mutex<VecDeque<Result<String, ProviderError>>>>,
    streams: Arc<Mutex<VecDeque<ScriptedStream>>>,
    complete_calls: Arc<AtomicUsize>,
    stream_calls: Arc<AtomicUsize>,
}

impl MockChatProvider {
    pub fn with_completion(result: Result<String, ProviderError>) -> Self {
        let provider = Self::default();
        provider.push_completion(result);
        provider
    }

    pub fn push_completion(&self, result: Result<String, ProviderError>) {
        self.completions.lock().unwrap().push_back(result);
    }

    pub fn push_stream(&self, result: Result<Vec<Result<String, ProviderError>>, ProviderError>) {
        self.streams.lock().unwrap().push_back(result);
    }

    pub fn complete_calls(&self) -> usize {
        self.complete_calls.load(Ordering::SeqCst)
    }

    pub fn stream_calls(&self) -> usize {
        self.stream_calls.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl ChatProvider for MockChatProvider {
    async fn complete(&self, _messages: Vec<ProviderMessage>) -> Result<String, ProviderError> {
        self.complete_calls.fetch_add(1, Ordering::SeqCst);
        self.completions
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| {
                Err(ProviderError::InvalidResponse(
                    "missing scripted completion".into(),
                ))
            })
    }

    async fn stream(
        &self,
        _messages: Vec<ProviderMessage>,
    ) -> Result<ProviderStream, ProviderError> {
        self.stream_calls.fetch_add(1, Ordering::SeqCst);
        let chunks = self
            .streams
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| {
                Err(ProviderError::InvalidResponse(
                    "missing scripted stream".into(),
                ))
            })?;
        Ok(Box::pin(stream::iter(chunks)))
    }
}

pub async fn test_pool() -> SqlitePool {
    let pool = db::connect_with_max_connections("sqlite::memory:", 1)
        .await
        .expect("connect test sqlite");
    db::run_migrations(&pool)
        .await
        .expect("run test migrations");
    pool
}

pub fn app_with_provider(pool: SqlitePool, provider: MockChatProvider) -> Router {
    let state = AppState::new(pool, Arc::new(provider));
    build_router(state, "http://localhost:3000").expect("build test router")
}

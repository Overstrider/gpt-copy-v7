use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod config;
pub mod db;
pub mod errors;
pub mod models;
pub mod provider;
pub mod repository;
pub mod routes;
pub mod validation;

use config::ConfigError;
use provider::ChatProvider;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub provider: Arc<dyn ChatProvider>,
}

impl AppState {
    pub fn new(pool: SqlitePool, provider: Arc<dyn ChatProvider>) -> Self {
        Self { pool, provider }
    }
}

pub fn build_router(state: AppState, frontend_origin: &str) -> Result<Router, ConfigError> {
    let origin: HeaderValue = frontend_origin
        .parse()
        .map_err(|_| ConfigError::InvalidFrontendOrigin(frontend_origin.to_owned()))?;

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    Ok(routes::router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http()))
}

pub fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tower_http=info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}

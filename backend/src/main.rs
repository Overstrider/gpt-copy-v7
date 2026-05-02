use std::sync::Arc;

use gpt_copy_v7_backend::{
    AppState, build_router, config::AppConfig, db, init_tracing, provider::OpenRouterProvider,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let config = AppConfig::load()?;
    let pool = db::connect_and_migrate(&config.database_url).await?;
    let provider = Arc::new(OpenRouterProvider::new(config.openrouter.clone()));
    let app = build_router(
        AppState::new(pool, provider),
        config.frontend_origin.as_str(),
    )?;

    let bind_address = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&bind_address).await?;
    tracing::info!(%bind_address, "starting backend");
    axum::serve(listener, app).await?;

    Ok(())
}

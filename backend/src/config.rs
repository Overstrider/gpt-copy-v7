use std::env;

pub const DEFAULT_OPENROUTER_MODEL: &str = "nvidia/nemotron-3-super-120b-a12b:free";
pub const DEFAULT_OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub frontend_origin: String,
    pub openrouter: OpenRouterConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenRouterConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: String,
    pub timeout_secs: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid BACKEND_PORT value: {0}")]
    InvalidPort(String),
    #[error("invalid FRONTEND_ORIGIN value: {0}")]
    InvalidFrontendOrigin(String),
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        Self::from_env_source(|key| env::var(key).ok())
    }

    pub fn from_env_source<F>(source: F) -> Result<Self, ConfigError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let host = non_empty(source("BACKEND_HOST")).unwrap_or_else(|| "127.0.0.1".to_owned());
        let port = match non_empty(source("BACKEND_PORT")) {
            Some(value) => value
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidPort(value))?,
            None => 8080,
        };
        let database_url = non_empty(source("DATABASE_URL"))
            .unwrap_or_else(|| "sqlite://backend/gpt-copy-v7.sqlite".to_owned());
        let frontend_origin = non_empty(source("FRONTEND_ORIGIN"))
            .unwrap_or_else(|| "http://localhost:3000".to_owned());
        let api_key = non_empty(source("OPENROUTER_API_KEY"));
        let model = non_empty(source("OPENROUTER_MODEL"))
            .unwrap_or_else(|| DEFAULT_OPENROUTER_MODEL.to_owned());
        let base_url = non_empty(source("OPENROUTER_BASE_URL"))
            .unwrap_or_else(|| DEFAULT_OPENROUTER_BASE_URL.to_owned());
        let timeout_secs = non_empty(source("OPENROUTER_TIMEOUT_SECS"))
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(60);

        Ok(Self {
            host,
            port,
            database_url,
            frontend_origin,
            openrouter: OpenRouterConfig {
                api_key,
                model,
                base_url,
                timeout_secs,
            },
        })
    }
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    })
}

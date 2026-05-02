use std::{pin::Pin, time::Duration};

use async_trait::async_trait;
use axum::http::StatusCode;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{config::OpenRouterConfig, models::Message};

pub type ProviderStream = Pin<Box<dyn Stream<Item = Result<String, ProviderError>> + Send>>;

#[async_trait]
pub trait ChatProvider: Send + Sync + 'static {
    async fn complete(&self, messages: Vec<ProviderMessage>) -> Result<String, ProviderError>;
    async fn stream(&self, messages: Vec<ProviderMessage>)
    -> Result<ProviderStream, ProviderError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderMessage {
    pub role: String,
    pub content: String,
}

impl ProviderMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_owned(),
            content: content.into(),
        }
    }

    pub fn from_message(message: &Message) -> Self {
        Self {
            role: message.role.as_str().to_owned(),
            content: message.content.clone(),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ProviderError {
    #[error("provider is not configured")]
    NotConfigured,
    #[error("provider rate limited request")]
    RateLimited,
    #[error("provider request timed out")]
    Timeout,
    #[error("provider returned invalid response: {0}")]
    InvalidResponse(String),
    #[error("provider stream interrupted")]
    InterruptedStream,
    #[error("provider request failed: {0}")]
    RequestFailed(String),
}

impl ProviderError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotConfigured => "provider_not_configured",
            Self::RateLimited => "provider_rate_limited",
            Self::Timeout => "provider_timeout",
            Self::InvalidResponse(_) => "provider_invalid_response",
            Self::InterruptedStream => "provider_stream_interrupted",
            Self::RequestFailed(_) => "provider_request_failed",
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotConfigured => StatusCode::SERVICE_UNAVAILABLE,
            Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
            Self::InvalidResponse(_) | Self::InterruptedStream | Self::RequestFailed(_) => {
                StatusCode::BAD_GATEWAY
            }
        }
    }

    pub fn public_message(&self) -> &'static str {
        match self {
            Self::NotConfigured => "chat provider is not configured",
            Self::RateLimited => "chat provider rate limited the request",
            Self::Timeout => "chat provider request timed out",
            Self::InvalidResponse(_) => "chat provider returned an invalid response",
            Self::InterruptedStream => "chat provider stream was interrupted",
            Self::RequestFailed(_) => "chat provider request failed",
        }
    }
}

#[derive(Clone)]
pub struct OpenRouterProvider {
    config: OpenRouterConfig,
    client: Client,
}

impl OpenRouterProvider {
    pub fn new(config: OpenRouterConfig) -> Self {
        let timeout = Duration::from_secs(config.timeout_secs);
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .expect("reqwest client configuration is valid");
        Self { config, client }
    }

    pub fn with_client(config: OpenRouterConfig, client: Client) -> Self {
        Self { config, client }
    }

    fn api_key(&self) -> Result<&str, ProviderError> {
        self.config
            .api_key
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .ok_or(ProviderError::NotConfigured)
    }

    fn completions_url(&self) -> String {
        format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        )
    }

    fn map_reqwest_error(error: reqwest::Error) -> ProviderError {
        if error.is_timeout() {
            ProviderError::Timeout
        } else {
            ProviderError::RequestFailed(error.to_string())
        }
    }

    async fn send_request(
        &self,
        messages: Vec<ProviderMessage>,
        stream: bool,
    ) -> Result<reqwest::Response, ProviderError> {
        let api_key = self.api_key()?;
        let request = OpenRouterRequest {
            model: self.config.model.clone(),
            messages,
            stream,
        };

        let response = self
            .client
            .post(self.completions_url())
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await
            .map_err(Self::map_reqwest_error)?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(ProviderError::RateLimited);
        }

        if !response.status().is_success() {
            return Err(ProviderError::InvalidResponse(format!(
                "provider status {}",
                response.status()
            )));
        }

        Ok(response)
    }
}

#[async_trait]
impl ChatProvider for OpenRouterProvider {
    async fn complete(&self, messages: Vec<ProviderMessage>) -> Result<String, ProviderError> {
        let response = self.send_request(messages, false).await?;
        let payload = response
            .json::<OpenRouterCompletionResponse>()
            .await
            .map_err(|error| {
                if error.is_timeout() {
                    ProviderError::Timeout
                } else {
                    ProviderError::InvalidResponse(error.to_string())
                }
            })?;

        payload
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .filter(|content| !content.is_empty())
            .ok_or_else(|| ProviderError::InvalidResponse("missing assistant content".to_owned()))
    }

    async fn stream(
        &self,
        messages: Vec<ProviderMessage>,
    ) -> Result<ProviderStream, ProviderError> {
        let response = self.send_request(messages, true).await?;
        let mut chunks = response.bytes_stream();

        Ok(Box::pin(async_stream::try_stream! {
            let mut buffer = String::new();
            let mut done = false;

            while let Some(chunk) = chunks.next().await {
                let chunk = chunk.map_err(Self::map_reqwest_error)?;
                let text = std::str::from_utf8(&chunk)
                    .map_err(|error| ProviderError::InvalidResponse(error.to_string()))?;
                buffer.push_str(text);

                while let Some((frame_end, separator_len)) = next_sse_frame(&buffer) {
                    let frame = buffer[..frame_end].to_owned();
                    buffer.drain(..frame_end + separator_len);

                    for data in frame_data_lines(&frame) {
                        if data == "[DONE]" {
                            done = true;
                            break;
                        }

                        if let Some(delta) = parse_stream_delta(&data)? {
                            yield delta;
                        }
                    }

                    if done {
                        break;
                    }
                }

                if done {
                    break;
                }
            }

            if !done {
                Err(ProviderError::InterruptedStream)?;
            }
        }))
    }
}

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<ProviderMessage>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenRouterCompletionResponse {
    choices: Vec<OpenRouterCompletionChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterCompletionChoice {
    message: OpenRouterMessage,
}

#[derive(Debug, Deserialize)]
struct OpenRouterMessage {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamResponse {
    choices: Vec<OpenRouterStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamChoice {
    delta: OpenRouterStreamDelta,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamDelta {
    content: Option<String>,
}

fn next_sse_frame(buffer: &str) -> Option<(usize, usize)> {
    match (buffer.find("\n\n"), buffer.find("\r\n\r\n")) {
        (Some(lf), Some(crlf)) if crlf < lf => Some((crlf, 4)),
        (Some(lf), _) => Some((lf, 2)),
        (None, Some(crlf)) => Some((crlf, 4)),
        (None, None) => None,
    }
}

fn frame_data_lines(frame: &str) -> Vec<String> {
    frame
        .lines()
        .filter_map(|line| {
            let line = line.trim_end_matches('\r');
            line.strip_prefix("data:").map(str::trim).map(str::to_owned)
        })
        .collect()
}

fn parse_stream_delta(data: &str) -> Result<Option<String>, ProviderError> {
    let payload: OpenRouterStreamResponse = serde_json::from_str(data)
        .map_err(|error| ProviderError::InvalidResponse(error.to_string()))?;

    let Some(choice) = payload.choices.into_iter().next() else {
        return Err(ProviderError::InvalidResponse(
            "missing stream choice".to_owned(),
        ));
    };

    Ok(choice.delta.content.filter(|content| !content.is_empty()))
}

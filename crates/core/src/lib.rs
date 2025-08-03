use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub provider: String,
    pub api_key: Option<String>,
    pub model: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, CoreError> {
        // TODO: Implement config loading from file/env
        Ok(Self {
            provider: "openai".to_string(),
            api_key: None,
            model: Some("gpt-4-turbo".to_string()),
        })
    }
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> anyhow::Result<String>;
    async fn stream(
        &self,
        prompt: &str,
    ) -> anyhow::Result<mpsc::Receiver<anyhow::Result<String>>>;
}

// Re-export providers-core for easy access
pub use zenocode_providers_core::{ProviderFactory, ProviderRegistry, PROVIDER_REGISTRY};

// Helper function to get a provider
pub fn get_provider(name: &str, config: &Config) -> Result<Box<dyn LLMProvider>, CoreError> {
    PROVIDER_REGISTRY
        .lock()
        .unwrap()
        .create_provider(name, config)
        .await
}

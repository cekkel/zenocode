use thiserror::Error;

pub mod config;
pub mod provider;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
}

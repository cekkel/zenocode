use crate::config::Config;
use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn query(&self, prompt: &str) -> anyhow::Result<String>;
    // TODO: Add streaming support
}

#[async_trait]
pub trait ProviderFactory: Send + Sync {
    async fn create(&self, config: &Config) -> anyhow::Result<Box<dyn LLMProvider>>;
    fn name(&self) -> &'static str;
}

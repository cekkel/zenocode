use async_stream::stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use zenocode_core::{Config, CoreError, LLMProvider};
use zenocode_providers_core::{ProviderFactory, register_provider};

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<RequestMessage>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct RequestMessage {
    role: String,
    content: String,
}

pub struct OpenAIClient {
    api_key: String,
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LLMProvider for OpenAIClient {
    async fn complete(&self, prompt: &str) -> anyhow::Result<String> {
        let request = OpenAIRequest {
            model: "gpt-4-turbo".to_string(),
            messages: vec![RequestMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            stream: false,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let response: OpenAIResponse = response.json().await?;
        Ok(response.choices[0].message.content.clone())
    }

    async fn stream(
        &self,
        prompt: &str,
    ) -> anyhow::Result<mpsc::Receiver<anyhow::Result<String>>> {
        // Simplified implementation - would use Server-Sent Events in real code
        let (tx, rx) = mpsc::channel(10);
        let content = self.complete(prompt).await?;
        for word in content.split_whitespace() {
            tx.send(Ok(word.to_string())).await?;
        }
        Ok(rx)
    }
}

// Factory implementation
pub struct OpenAIFactory;

#[async_trait::async_trait]
impl ProviderFactory for OpenAIFactory {
    async fn create(&self, config: &Config) -> Result<Box<dyn LLMProvider>, CoreError> {
        let api_key = config.api_key.clone().ok_or_else(|| {
            CoreError::ConfigError("OpenAI API key not configured".to_string())
        })?;
        Ok(Box::new(OpenAIClient::new(&api_key)))
    }

    fn name(&self) -> &'static str {
        "openai"
    }
}

// Register on library load
#[ctor::ctor]
fn register() {
    register_provider!(OpenAIFactory);
}
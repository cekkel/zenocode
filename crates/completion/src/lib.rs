use anyhow::Context;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use zenocode_core::config::Config;
use zenocode_core::provider::{LLMProvider, ProviderFactory};

pub struct ProviderRegistry {
    factories: HashMap<String, Arc<dyn ProviderFactory>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, factory: Arc<dyn ProviderFactory>) {
        self.factories.insert(factory.name().to_string(), factory);
    }

    pub async fn create_provider(
        &self,
        name: &str,
        config: &Config,
    ) -> anyhow::Result<Box<dyn LLMProvider>> {
        self.factories
            .get(name)
            .with_context(|| format!("Provider '{}' not found", name))?
            .create(config)
            .await
    }

    pub fn available_providers(&self) -> Vec<&'static str> {
        self.factories.values().map(|f| f.name()).collect()
    }
}

// Global registry instance
lazy_static::lazy_static! {
    pub static ref PROVIDER_REGISTRY: Mutex<ProviderRegistry> = Mutex::new(ProviderRegistry::new());
}

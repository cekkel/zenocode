use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zenocode_core::{Config, CoreError, LLMProvider};

#[async_trait]
pub trait ProviderFactory: Send + Sync {
    async fn create(&self, config: &Config) -> Result<Box<dyn LLMProvider>, CoreError>;
    fn name(&self) -> &'static str;
}

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

    pub fn create_provider(
        &self,
        name: &str,
        config: &Config,
    ) -> Result<Box<dyn LLMProvider>, CoreError> {
        self.factories
            .get(name)
            .ok_or_else(|| CoreError::ProviderError(format!("Provider '{}' not found", name)))?
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

// Macro for easy provider registration
#[macro_export]
macro_rules! register_provider {
    ($factory:expr) => {
        $crate::PROVIDER_REGISTRY
            .lock()
            .unwrap()
            .register(Arc::new($factory));
    };
}
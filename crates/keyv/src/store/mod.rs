use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait Store {
    async fn get<V>(&self, key: &str) -> Result<V, String>
    where
        V: for<'de> Deserialize<'de> + Send + Sync;

    async fn set(&self, key: &str, value: &str) -> Result<bool, String>;
}

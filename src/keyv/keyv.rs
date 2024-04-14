use std::sync::Arc;

use serde::Serialize;
use serde_json::{json, Value};

use crate::store::Store;

use super::KeyvError;

pub struct Keyv {
    store: Arc<dyn Store>,
}

impl Keyv {
    pub async fn try_new<S: Store + 'static>(store: S) -> Result<Self, KeyvError> {
        store.initialize().await?;
        Ok(Self {
            store: Arc::new(store),
        })
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: T) -> Result<(), KeyvError> {
        Ok(self.store.set(key, json!(value), None).await?)
    }

    pub async fn set_with_ttl<T: Serialize>(
        &self,
        key: &str,
        value: T,
        ttl: u64,
    ) -> Result<(), KeyvError> {
        Ok(self.store.set(key, json!(value), Some(ttl)).await?)
    }

    pub async fn get(&self, key: &str) -> Result<Option<Value>, KeyvError> {
        Ok(self.store.get(key).await?)
    }

    pub async fn remove(&self, key: &str) -> Result<(), KeyvError> {
        Ok(self.store.remove(key).await?)
    }

    pub async fn remove_many<T: AsRef<str> + Sync>(&self, keys: &[T]) -> Result<(), KeyvError> {
        let keys: Vec<&str> = keys.iter().map(|k| k.as_ref()).collect();
        Ok(self.store.remove_many(&keys).await?)
    }

    pub async fn clear(&self) -> Result<(), KeyvError> {
        Ok(self.store.clear().await?)
    }
}

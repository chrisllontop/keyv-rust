use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{Store, StoreError};

pub struct InMemoryStore {
    db: Mutex<HashMap<String, Value>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            db: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Store for InMemoryStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        let db_lock = self.db.lock().await;
        Ok(db_lock.get(key).cloned())
    }

    async fn set(&self, key: &str, value: Value, _ttl: Option<u64>) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().await;
        db_lock.insert(key.to_string(), value.clone());
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().await;
        db_lock.remove(key);
        Ok(())
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().await;
        for key in keys {
            db_lock.remove(&key.to_string());
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().await;
        db_lock.clear();
        Ok(())
    }
}

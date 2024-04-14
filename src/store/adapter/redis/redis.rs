use std::sync::Arc;

use async_trait::async_trait;
use redis::{Client, Commands};
use serde_json::Value;

use crate::{Store, StoreError};

pub struct RedisStore {
    pub(crate) client: Arc<Client>,
    pub(crate) default_ttl: Option<u64>,
}

#[async_trait]
impl Store for RedisStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        Ok(()) // Redis doesn't require initialization like a DB schema.
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        let value: Option<String> = conn
            .get(key)
            .map_err(|e| StoreError::QueryError(e.to_string()))?;
        match value {
            Some(val) => Ok(serde_json::from_str(&val)
                .map(Some)
                .map_err(|e| StoreError::SerializationError { source: e })?),
            None => Ok(None),
        }
    }

    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), StoreError> {
        let ttl = ttl.or(self.default_ttl);
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        if let Some(expire) = ttl {
            conn.set_ex(key, value_str, expire as usize)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        } else {
            conn.set(key, value_str)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        }
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        conn.del(key)
            .map_err(|e| StoreError::QueryError(e.to_string()))?;
        Ok(())
    }

    async fn remove_many<T: AsRef<str> + Sync>(&self, keys: &[T]) -> Result<(), StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        let keys_str: Vec<&str> = keys.iter().map(|k| k.as_ref()).collect();

        conn.del(keys_str)
            .map_err(|e| StoreError::QueryError(e.to_string()))?;
        Ok(())
    }

    async fn clear(&self) -> Result<(), StoreError> {
        Ok(())
    }
}

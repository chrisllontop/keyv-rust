use std::sync::Arc;

use async_trait::async_trait;
use redis::{Client, Commands};
use serde_json::Value;

use crate::{Store, StoreError};

pub struct RedisStore {
    pub(crate) client: Arc<Client>,
    pub(crate) default_ttl: Option<u64>,
    pub(crate) namespace: Option<String>,
}
impl RedisStore {
    fn get_key(&self, key: &str) -> String {
        if let Some(ref ns) = self.namespace {
            format!("{}:{}", ns, key)
        } else {
            key.to_string()
        }
    }
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
            .get(self.get_key(key))
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
        let namespaced_key = self.get_key(key);
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        if let Some(expire) = ttl {
            conn.set_ex::<_, _, ()>(&namespaced_key, value_str, expire)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        } else {
            conn.set::<_, _, ()>(&namespaced_key, value_str)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        }
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
        conn.del::<_, ()>(self.get_key(key))
            .map_err(|e| StoreError::QueryError(e.to_string()))?;
        Ok(())
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;

        let namespaced_keys: Vec<String> = keys.iter().map(|key| self.get_key(key)).collect();

        if !namespaced_keys.is_empty() {
            conn.del::<_, ()>(namespaced_keys)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), StoreError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(|e| StoreError::ConnectionError(e.to_string()))?;

        if let Some(ref ns) = self.namespace {
            let pattern = format!("{}:*", ns);
            let keys: Vec<String> = conn
                .keys(&pattern)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
            if !keys.is_empty() {
                conn.del::<_, ()>(keys)
                    .map_err(|e| StoreError::QueryError(e.to_string()))?;
            }
        } else {
            redis::cmd("FLUSHDB")
                .query::<()>(&mut conn)
                .map_err(|e| StoreError::QueryError(e.to_string()))?;
        }
        Ok(())
    }
}

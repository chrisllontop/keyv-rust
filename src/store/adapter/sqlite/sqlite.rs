use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use sqlx::SqlitePool;

use crate::{Store, StoreError};

pub struct SqliteStore {
    pub(crate) pool: Arc<SqlitePool>,
    pub(crate) table_name: String,
}

impl SqliteStore {
    fn get_table_name(&self) -> String {
        self.table_name.clone()
    }
}

#[async_trait]
impl Store for SqliteStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            self.get_table_name()
        );

        sqlx::query(&sql).execute(&*self.pool).await.map_err(|e| {
            StoreError::QueryError(format!(
                "Failed to initialize the database table: {}",
                e.to_string()
            ))
        })?;

        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        let query = format!("SELECT value FROM {} WHERE key = ?", self.get_table_name());
        let result = sqlx::query_as::<_, (String,)>(query.as_str())
            .bind(key)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to fetch the value".to_string()))?;

        Ok(result
            .map(|(value,)| serde_json::from_str(&value).ok())
            .flatten())
    }

    async fn set(&self, key: &str, value: Value, _ttl: Option<u64>) -> Result<(), StoreError> {
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        let sql = format!(
            "INSERT INTO {} (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = EXCLUDED.value",
            self.get_table_name()
        );
        sqlx::query(&sql)
            .bind(key)
            .bind(value_str)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to set the value".to_string()))?;

        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let query = format!("DELETE FROM {} WHERE key = ?", self.get_table_name());
        sqlx::query(&query)
            .bind(key)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to remove the key".to_string()))?;

        Ok(())
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        let query = format!(
            "DELETE FROM {} WHERE key IN ({})",
            self.get_table_name(),
            keys.iter().map(|_| "?").collect::<Vec<&str>>().join(",")
        );

        let mut query = sqlx::query(&query);
        for key in keys {
            query = query.bind(key);
        }

        query.execute(&*self.pool).await.map_err(|e| {
            StoreError::QueryError(format!("Failed to remove the keys: {}", e.to_string()))
        })?;

        Ok(())
    }

    async fn clear(&self) -> Result<(), StoreError> {
        let query = format!("DELETE FROM {}", self.get_table_name());

        sqlx::query(&query)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to clear the table".to_string()))?;

        Ok(())
    }
}

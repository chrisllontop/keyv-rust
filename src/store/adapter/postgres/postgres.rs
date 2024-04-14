use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{Store, StoreError};

pub struct PostgresStore {
    pub(crate) pool: Arc<PgPool>,
    pub(crate) table_name: String,
    pub(crate) schema: Option<String>,
}
impl PostgresStore {
    fn get_table_name(&self) -> String {
        match &self.schema {
            Some(schema) => format!("{}.{}", schema, self.table_name),
            None => self.table_name.clone(),
        }
    }
}

#[async_trait]
impl Store for PostgresStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        if let Some(ref schema) = self.schema {
            let create_schema_sql = format!("CREATE SCHEMA IF NOT EXISTS {}", schema);
            sqlx::query(&create_schema_sql)
                .execute(&*self.pool)
                .await
                .map_err(|e| {
                    StoreError::QueryError(format!(
                        "Failed to create the schema '{}': {}",
                        schema,
                        e.to_string()
                    ))
                })?;
        }

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            key VARCHAR PRIMARY KEY,
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
        let query = format!("SELECT value FROM {} WHERE key = $1", self.get_table_name());
        let result = sqlx::query(&query)
            .bind(key)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to fetch the value".to_string()))?;

        Ok(result
            .map(|row| serde_json::from_str(row.get("value")).ok())
            .flatten())
    }

    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), StoreError> {
        if ttl.is_some() {
            log::warn!("Postgres store does not support TTL");
        }

        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        let sql = format!(
            "INSERT INTO {} (key, value) VALUES ($1, $2) ON CONFLICT(key) DO UPDATE SET value = EXCLUDED.value",
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
        let query = format!("DELETE FROM {} WHERE key = $1", self.get_table_name());
        sqlx::query(&query)
            .bind(key)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to remove the key".to_string()))?;

        Ok(())
    }

    async fn remove_many<T: AsRef<str> + Sync>(&self, keys: &[T]) -> Result<(), StoreError> {
        let keys_str: Vec<&str> = keys.iter().map(|k| k.as_ref()).collect();

        let query = format!("DELETE FROM {} WHERE key = ANY($1)", self.get_table_name());

        sqlx::query(&query)
            .bind(&keys_str)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to remove the keys".to_string()))?;

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

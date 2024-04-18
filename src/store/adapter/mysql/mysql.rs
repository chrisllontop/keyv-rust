use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use sqlx::{mysql::MySqlPool, Row};

use crate::{Store, StoreError};

pub struct MySqlStore {
    pub(crate) pool: Arc<MySqlPool>,
    pub(crate) table_name: String,
}

/// Builder for creating a `MySqlStore`.
///
/// This builder allows for configuring a `MySqlStore` with custom
/// settings such as a specific database URI, an existing connection pool,
/// and a table name. It provides a flexible way to initialize the store
/// depending on the application's requirements.
///
/// # Examples
///
/// ## Initializing with a Database URI
///
/// ```rust,no_run
/// # use keyv::adapter::mysql::{MySqlStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main(){
/// let store = MySqlStoreBuilder::new()
///     .uri("mysql://username:password@localhost/database")
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
/// }
/// ```
///
/// ## Using an Existing Connection Pool
///
/// ```rust,no_run
/// # use keyv::adapter::mysql::{MySqlStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let pool: Arc<sqlx::MySqlPool> = Arc::new(sqlx::mysql::MySqlPoolOptions::new()
///     .connect("mysql://username:password@localhost/database").await.unwrap());
///
/// let store = MySqlStoreBuilder::new()
///     .pool(pool)
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
/// }
/// ```
impl MySqlStore {
    fn get_table_name(&self) -> String {
        self.table_name.clone()
    }
}

#[async_trait]
impl Store for MySqlStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            `key` VARCHAR(255) PRIMARY KEY,
            `value` TEXT NOT NULL
        ) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci",
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
        let query = format!(
            "SELECT `value` FROM {} WHERE `key` = ?",
            self.get_table_name()
        );
        let result = sqlx::query(&query)
            .bind(key)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to fetch the value".to_string()))?;

        Ok(result.and_then(|row| serde_json::from_str(row.get("value")).ok()))
    }

    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), StoreError> {
        if ttl.is_some() {
            log::warn!("TTL is not supported by the MySQL store");
        }

        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        let sql = format!(
            "INSERT INTO {} (`key`, `value`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `value` = VALUES(`value`)",
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
        let query = format!("DELETE FROM {} WHERE `key` = ?", self.get_table_name());
        sqlx::query(&query)
            .bind(key)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to remove the key".to_string()))?;

        Ok(())
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        let keys_placeholder: String = keys.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            "DELETE FROM {} WHERE `key` IN ({})",
            self.get_table_name(),
            keys_placeholder
        );

        let mut query_builder = sqlx::query(&query);
        for key in keys {
            query_builder = query_builder.bind(key);
        }

        query_builder
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

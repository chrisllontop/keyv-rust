use std::sync::Arc;

use async_trait::async_trait;
use keyv::{Store, StoreError};
use serde_json::Value;
use sqlx::{PgPool, Row};

pub struct PostgresStore {
    pub(crate) pool: Arc<PgPool>,
    pub(crate) table_name: String,
}

#[async_trait]
impl Store for PostgresStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            key VARCHAR PRIMARY KEY,
            value TEXT NOT NULL,
            ttl BIGINT
        )",
            self.table_name
        );

        sqlx::query(&sql).execute(&*self.pool).await.map_err(|_| {
            StoreError::QueryError("Failed to initialize the database table".to_string())
        })?;

        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        let query = format!("SELECT value FROM {} WHERE key = $1", self.table_name);
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
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        let sql = format!(
            "INSERT INTO {} (key, value, ttl) VALUES ($1, $2, $3) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, ttl = EXCLUDED.ttl",
            self.table_name
        );
        sqlx::query(&sql)
            .bind(key)
            .bind(value_str)
            .bind(ttl.map(|t| t as i64))
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to set the value".to_string()))?;

        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let query = format!("DELETE FROM {} WHERE key = $1", self.table_name);
        sqlx::query(&query)
            .bind(key)
            .execute(&*self.pool)
            .await
            .map_err(|_| StoreError::QueryError("Failed to remove the key".to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::builder::PostgresStoreBuilder;

    use super::StoreError;
    use async_trait::async_trait;
    use keyv::Keyv;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::Mutex;

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
            let db_lock = self.db.lock().unwrap();
            Ok(db_lock.get(key).cloned())
        }

        async fn set(&self, key: &str, value: Value, _ttl: Option<u64>) -> Result<(), StoreError> {
            let mut db_lock = self.db.lock().unwrap();
            db_lock.insert(key.to_string(), value.clone());
            Ok(())
        }

        async fn remove(&self, key: &str) -> Result<(), StoreError> {
            let mut db_lock = self.db.lock().unwrap();
            db_lock.remove(key);
            Ok(())
        }
    }

    use super::*;
    #[tokio::test]
    async fn test_keyv_postgres() {
        let store = PostgresStoreBuilder::new()
            .uri("postgresql://postgres:postgres@localhost:5432/postgres")
            .build()
            .await
            .unwrap();

        let keyv = Keyv::try_new(store).await.unwrap();
        keyv.set("number", 42).await.unwrap();
        keyv.set("number", 10).await.unwrap();
        keyv.set("array", vec!["hola", "test"]).await.unwrap();
        keyv.set("string", "life long").await.unwrap();

        match keyv.get("number").await.unwrap() {
            Some(number) => {
                let number: i32 = serde_json::from_value(number).unwrap();
                assert_eq!(number, 10);
            }
            None => assert!(false),
        }

        match keyv.get("string").await.unwrap() {
            Some(string) => {
                let string: String = serde_json::from_value(string).unwrap();
                assert_eq!(string, "life long");
            }
            None => assert!(false),
        }

        match keyv.get("array").await.unwrap() {
            Some(array) => {
                let array: Vec<String> = serde_json::from_value(array).unwrap();
                assert_eq!(array, vec!["hola".to_string(), "test".to_string()])
            }
            None => assert!(false),
        }
    }
}

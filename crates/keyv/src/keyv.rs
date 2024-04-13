use serde::Serialize;
use serde_json::{json, Value};

use crate::store::{KeyvError, Store};

pub struct Keyv<S: Store> {
    store: S,
}

impl<S> Keyv<S>
where
    S: Store,
{
    pub fn new(store: S) -> Self {
        Keyv { store }
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: T) -> Result<(), KeyvError> {
        self.store.set(key, json!(value), None).await
    }

    pub async fn set_with_ttl<T: Serialize>(
        &self,
        key: &str,
        value: T,
        ttl: u64,
    ) -> Result<(), KeyvError> {
        self.store.set(key, json!(value), Some(ttl)).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<Value>, KeyvError> {
        self.store.get(key).await
    }

    pub async fn remove(&self, key: &str) -> Result<(), KeyvError> {
        self.store.remove(key).await
    }
}

#[cfg(test)]
mod tests {

    use super::KeyvError;
    use async_trait::async_trait;
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
        async fn initialize(&self) -> Result<(), KeyvError> {
            Ok(())
        }

        async fn get(&self, key: &str) -> Result<Option<Value>, KeyvError> {
            let db_lock = self.db.lock().unwrap();
            Ok(db_lock.get(key).cloned())
        }

        async fn set(&self, key: &str, value: Value, _ttl: Option<u64>) -> Result<(), KeyvError> {
            let mut db_lock = self.db.lock().unwrap();
            db_lock.insert(key.to_string(), value.clone());
            Ok(())
        }

        async fn remove(&self, key: &str) -> Result<(), KeyvError> {
            let mut db_lock = self.db.lock().unwrap();
            db_lock.remove(key);
            Ok(())
        }
    }

    use super::*;
    #[tokio::test]
    async fn test_keyv() {
        let store = InMemoryStore::new();
        let keyv = Keyv::new(store);
        keyv.set("number", 42).await.unwrap();
        keyv.set("number", 10).await.unwrap();
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
    }
}

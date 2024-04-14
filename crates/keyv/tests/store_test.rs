use keyv::{Keyv, Store, StoreError};

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

    async fn clear(&self) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().unwrap();
        db_lock.clear();
        Ok(())
    }

    async fn remove_many<T: AsRef<str> + Sync>(&self, keys: &[T]) -> Result<(), StoreError> {
        let mut db_lock = self.db.lock().unwrap();
        for key in keys {
            db_lock.remove(key.as_ref());
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_keyv() {
    let store = InMemoryStore::new();
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

    match keyv.remove_many(&["number", "string"]).await {
        Ok(_) => {}
        Err(_) => assert!(false),
    }
}

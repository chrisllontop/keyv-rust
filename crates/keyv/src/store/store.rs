use async_trait::async_trait;
use serde_json::Value;

use super::StoreError;

#[async_trait]
pub trait Store: Send + Sync {
    async fn initialize(&self) -> Result<(), StoreError>;

    // Retrieves a value based on a key.
    // The value is expected to be serialized into a string (or potentially bytes),
    // hence the return type is Result<Option<Value>, KeyvError>.
    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError>;
    //Evel to use this
    // async fn get<V>(&self, key: &str) -> Result<Option<V>, KeyvError>
    // where
    //     V: DeserializeOwned + Send + Sync;

    // Sets a value for a given key.
    // Both key and value are strings for simplicity, but could be more complex types
    // (with proper serialization/deserialization).
    // The function returns Result<(), KeyvError> to indicate success or error.
    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), StoreError>;

    /// Removes a key from the store.
    /// Returns Result<(), KeyvError> to indicate the operation was successful or encountered an error.
    async fn remove(&self, key: &str) -> Result<(), StoreError>;

    async fn remove_many<T: AsRef<str> + Sync>(&self, keys: &[T]) -> Result<(), StoreError>;

    async fn clear(&self) -> Result<(), StoreError>;
}

use async_trait::async_trait;
use serde_json::Value;

use super::KeyvError;

#[async_trait]
pub trait Store {
    async fn initialize(&self) -> Result<(), KeyvError>;

    // Retrieves a value based on a key.
    // The value is expected to be serialized into a string (or potentially bytes),
    // hence the return type is Result<Option<Value>, KeyvError>.
    async fn get(&self, key: &str) -> Result<Option<Value>, KeyvError>;
    //Evel to use this
    // async fn get<V>(&self, key: &str) -> Result<Option<V>, KeyvError>
    // where
    //     V: DeserializeOwned + Send + Sync;

    // Sets a value for a given key.
    // Both key and value are strings for simplicity, but could be more complex types
    // (with proper serialization/deserialization).
    // The function returns Result<(), KeyvError> to indicate success or error.
    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), KeyvError>;

    /// Removes a key from the store.
    /// Returns Result<(), KeyvError> to indicate the operation was successful or encountered an error.
    async fn remove(&self, key: &str) -> Result<(), KeyvError>;
}

impl<S> From<S> for Box<dyn Store>
where
    S: Store + 'static,
{
    fn from(store: S) -> Box<dyn Store> {
        Box::new(store)
    }
}

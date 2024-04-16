use async_trait::async_trait;
use serde_json::Value;

use super::StoreError;

#[async_trait]
pub trait Store: Send + Sync {
    /// Initializes the storage backend.
    /// This method should perform any necessary setup for the storage backend, such as
    /// establishing database connections or ensuring the existence of required files or schemas.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(StoreError)` if initialisation fails.
    async fn initialize(&self) -> Result<(), StoreError>;

    /// Retrieves a value associated with a given key from the store.
    ///
    /// # Arguments
    /// - `key`: A string slice that holds the key for the value to be retrieved.
    ///
    /// # Returns
    /// - `Ok(Some(Value))` if the key exists and the value is successfully retrieved.
    /// - `Ok(None)` if the key does not exist.
    /// - `Err(StoreError)` if there is an error retrieving the value.
    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError>;

    /// Sets a value for a given key in the store, with an optional time-to-live (TTL).
    ///
    /// # Arguments
    /// - `key`: The key under which the value is stored.
    /// - `value`: The value to set, represented as a `serde_json::Value`.
    /// - `ttl`: An optional u64 representing the time-to-live in seconds.
    ///
    /// # Returns
    /// - `Ok(())` if the value is successfully set.
    /// - `Err(StoreError)` if there is an error setting the value.
    async fn set(&self, key: &str, value: Value, ttl: Option<u64>) -> Result<(), StoreError>;

    /// Removes a value associated with a given key from the store.
    ///
    /// # Arguments
    /// - `key`: A string slice that holds the key for the value to be removed.
    ///
    /// # Returns
    /// - `Ok(())` if the key exists and the value is successfully removed.
    /// - `Err(StoreError)` if there is an error removing the value.
    async fn remove(&self, key: &str) -> Result<(), StoreError>;

    /// Removes multiple values associated with the given keys from the store.
    ///
    /// # Arguments
    /// - `keys`: A slice of string slices representing the keys for the values to be removed.
    ///
    /// # Returns
    /// - `Ok(())` if the values are successfully removed.
    /// - `Err(StoreError)` if there is an error removing the values.
    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError>;

    /// Clears all values from the store.
    ///
    /// # Returns
    /// - `Ok(())` if the store is successfully cleared.
    /// - `Err(StoreError)` if there is an error clearing the store.
    async fn clear(&self) -> Result<(), StoreError>;
}

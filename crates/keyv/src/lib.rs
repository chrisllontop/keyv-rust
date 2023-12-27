mod store;

use crate::store::Store;

struct Keyv<T: Store> {
    store: T,
}

impl<T: Store> Keyv<T> {
    pub fn new(store: T) -> Self {
        Keyv { store }
    }

    pub async fn get<V>(&self, key: &str) -> Result<V, String>
    where
        V: for<'de> serde::Deserialize<'de> + Send + Sync,
    {
        self.store.get(key).await
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<bool, String> {
        self.store.set(key, value).await
    }
}

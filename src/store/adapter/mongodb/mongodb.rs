use async_trait::async_trait;
use mongodb::{
    bson::{doc, Bson, Document},
    Client, Collection,
};
use serde_json::Value;
use std::sync::Arc;

use crate::{Store, StoreError};

pub struct MongoStore {
    pub(crate) client: Arc<Client>,
    pub(crate) database_name: String,
    pub(crate) collection_name: String,
}

impl MongoStore {
    fn get_collection(&self) -> Collection<Document> {
        self.client
            .database(&self.database_name)
            .collection(&self.collection_name)
    }
}

#[async_trait]
impl Store for MongoStore {
    async fn initialize(&self) -> Result<(), StoreError> {
        // MongoDB creates databases and collections automatically when you insert data,
        // so explicit creation is not needed.
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Value>, StoreError> {
        let coll = self.get_collection();
        let filter = doc! { "key": key };
        let result = coll
            .find_one(filter, None)
            .await
            .map_err(|e| StoreError::QueryError(e.to_string()))?;

        result
            .map_or(Ok(None), |doc| {
                doc.get("value")
                    .and_then(Bson::as_str)
                    .map(|s| serde_json::from_str::<Value>(s))
                    .transpose()
            })
            .map_err(|e| StoreError::SerializationError { source: e.into() })
    }

    async fn set(&self, key: &str, value: Value, _: Option<u64>) -> Result<(), StoreError> {
        let coll = self.get_collection();
        let value_str = serde_json::to_string(&value)
            .map_err(|e| StoreError::SerializationError { source: e })?;

        let doc = doc! {
            "key": key,
            "value": value_str
        };

        let replace_options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();

        coll.replace_one(doc! { "key": key }, doc, replace_options)
            .await
            .map(|replace_result| {
                if replace_result.upserted_id.is_some() {
                    log::info!("A new document was upserted");
                }
                ()
            })
            .map_err(|e| {
                StoreError::QueryError(format!("Failed to set the value: {}", e.to_string()))
            })
    }

    async fn remove(&self, key: &str) -> Result<(), StoreError> {
        let coll = self.get_collection();
        coll.delete_one(doc! { "key": key }, None)
            .await
            .map(|_| ())
            .map_err(|_| StoreError::QueryError("Failed to remove the key".to_string()))
    }

    async fn remove_many(&self, keys: &[&str]) -> Result<(), StoreError> {
        let coll = self.get_collection();
        coll.delete_many(doc! { "key": { "$in": keys } }, None)
            .await
            .map(|_| ())
            .map_err(|_| StoreError::QueryError("Failed to remove the keys".to_string()))
    }

    async fn clear(&self) -> Result<(), StoreError> {
        let coll = self.get_collection();
        coll.delete_many(doc! {}, None)
            .await
            .map(|_| ())
            .map_err(|_| StoreError::QueryError("Failed to clear the collection".to_string()))
    }
}

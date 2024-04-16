pub use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;

use crate::{StoreError, DEFAUTL_TABLE_NAME};

use super::MySqlStore;

pub struct MySqlStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<MySqlPool>>,
    table_name: String,
}

impl MySqlStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: DEFAUTL_TABLE_NAME.to_string(),
        }
    }

    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = table.into();
        self
    }

    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    pub fn pool(mut self, pool: Arc<MySqlPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub async fn build(self) -> Result<MySqlStore, StoreError> {
        let pool = match self.pool {
            Some(pool) => pool,
            None => {
                let uri = self
                    .uri
                    .expect("MySqlStore requires either a URI or an existing pool to be set");
                Arc::new(MySqlPoolOptions::new().connect(&uri).await.map_err(|_| {
                    StoreError::ConnectionError("Failed to connect to the database".to_string())
                })?)
            }
        };

        Ok(MySqlStore {
            pool,
            table_name: self.table_name,
        })
    }
}

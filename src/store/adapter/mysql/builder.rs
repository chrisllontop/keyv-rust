pub use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::MySqlStore;

pub struct MySqlStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<MySqlPool>>,
    table_name: Option<String>,
}

impl MySqlStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: None,
        }
    }

    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = Some(table.into());
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
        let table_name = match &self.table_name {
            Some(table_name) => table_name.to_string(),
            None => {
                log::warn!("Table name not set, using default table name");
                DEFAUTL_NAMESPACE_NAME.to_string()
            }
        };

        Ok(MySqlStore { pool, table_name })
    }
}

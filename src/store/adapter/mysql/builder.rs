pub use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::sync::Arc;

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::MySqlStore;

/// Builder for creating a `MySqlStore`.
///
/// This builder allows for configuring a `MySqlStore` with custom settings such
/// as a specific database URI, an existing connection pool, and a table name. It
/// provides a flexible way to initialize the store depending on the application's
/// requirements.
///
/// # Examples
///
/// ## Initializing with a Database URI
///
/// ```rust,no_run
/// # use keyv::adapter::mysql::{MySqlStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main(){
/// let store = MySqlStoreBuilder::new()
///     .uri("mysql://username:password@localhost/database")
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
/// }
/// ```
///
/// ## Using an Existing Connection Pool
///
/// ```rust,no_run
/// # use keyv::adapter::mysql::{MySqlStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let pool: Arc<sqlx::MySqlPool> = Arc::new(sqlx::mysql::MySqlPoolOptions::new()
///     .connect("mysql://username:password@localhost/database").await.unwrap());
///
/// let store = MySqlStoreBuilder::new()
///     .pool(pool)
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
/// }
/// ```
pub struct MySqlStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<MySqlPool>>,
    table_name: Option<String>,
}

/// Creates a new builder instance with default configuration.
///
/// Initializes the builder with the default table name and no predefined URI or connection pool.
/// The default table name is defined by `DEFAULT_NAMESPACE_NAME`.
impl MySqlStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: None,
        }
    }

    /// Sets the table name for the `MySqlStore`.
    ///
    /// This method configures the table name to be used by the store. If not set,
    /// the default table name is used.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table used to store key-value pairs.
    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = Some(table.into());
        self
    }

    /// Sets the database URI for connecting to the MySQL database.
    ///
    /// This method configures the database URI. It's required if no existing
    /// connection pool is provided.
    ///
    /// # Arguments
    ///
    /// * `uri` - The database URI string.
    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Uses an existing connection pool for the `MySqlStore`.
    ///
    /// This method allows for using an already configured `MySqlPool`. If set,
    /// the `uri` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `pool` - Shared reference to an existing `MySqlPool`.
    pub fn pool(mut self, pool: Arc<MySqlPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Builds the `MySqlStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates a `MySqlStore` instance. It requires
    /// either a database URI or an existing connection pool to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized
    /// `MySqlStore`. On failure, it returns a `StoreError` indicating what went
    /// wrong during the initialization.
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

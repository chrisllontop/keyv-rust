use std::sync::Arc;

pub use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::SqliteStore;

/// Builder for creating a `SqliteStore`.
///
/// This builder allows for configuring a `SqliteStore` with custom
/// settings such as a specific database file URI and a table name.
/// It provides a flexible way to initialize the store depending on the
/// application's requirements.
///
/// # Examples
///
/// ## Initializing with a Database File URI
///
/// ```rust,no_run
/// # use keyv::adapter::sqlite::{SqliteStoreBuilder};
/// # #[tokio::main]
/// # async fn main(){
/// let store = SqliteStoreBuilder::new()
///     .uri("sqlite::memory:")
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
///  }
/// ```
///
/// ## Using an Existing Connection Pool
///
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use keyv::adapter::sqlite::{SqliteStoreBuilder};
/// # use keyv::adapter::sqlite::SqlitePoolOptions;
/// # #[tokio::main]
/// # async fn main() {
/// let pool: Arc<sqlx::SqlitePool> = Arc::new(SqlitePoolOptions::new()
///     .connect("sqlite::memory:").await.unwrap());
///
/// let store = SqliteStoreBuilder::new()
///     .pool(pool)
///     .table_name("custom_table_name")
///     .build()
///     .await.unwrap();
/// }
/// ```
pub struct SqliteStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<SqlitePool>>,
    table_name: Option<String>,
}

impl SqliteStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: None,
        }
    }

    /// Sets the table name for the `SqliteStore`.
    ///
    /// This method configures the table name to be used by the store. If not set,
    /// `DEFAULT_TABLE_NAME` from the configuration will be used.
    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = Some(table.into());
        self
    }

    /// Sets the database URI for connecting to the SQLite database.
    ///
    /// This method configures the database URI. It's required if no existing connection pool is provided.
    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Uses an existing connection pool for the `SqliteStore`.
    ///
    /// This method allows for using an already configured `SqlitePool`. If set,
    /// the `uri` option is ignored.
    pub fn pool(mut self, pool: Arc<SqlitePool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Builds the `SqliteStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates an `SqliteStore` instance.
    /// It requires either a database URI or an existing connection pool to be set.
    ///
    /// # Returns
    /// This method returns a `Result` which, on success, contains the initialized `SqliteStore`.
    /// On failure, it returns a `StoreError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<SqliteStore, StoreError> {
        let pool = match self.pool {
            Some(pool) => pool,
            None => {
                let uri = self
                    .uri
                    .expect("SqliteStore requires either a URI or an existing pool to be set");
                Arc::new(SqlitePoolOptions::new().connect(&uri).await.map_err(|_| {
                    StoreError::ConnectionError("Failed to connect to the database".to_string())
                })?)
            }
        };

        let table_name = self.table_name.unwrap_or_else(|| {
            log::warn!("Table name not set, using default table name");
            DEFAUTL_NAMESPACE_NAME.to_string()
        });

        Ok(SqliteStore { pool, table_name })
    }
}

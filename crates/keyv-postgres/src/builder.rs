use std::sync::Arc;

use keyv::{StoreError, DEFAUTL_TABLE_NAME};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::store::PostgresStore;
/// Builder for creating a `PostgresStore`.
///
/// This builder allows for configuring a `PostgresStore` with custom
/// settings such as a specific database URI, an existing connection pool,
/// and a table name. It provides a flexible way to initialize the store
/// depending on the application's requirements.
///
/// # Examples
///
/// ## Initializing with a Database URI
///
/// ```no_run
/// # use your_crate::{PostgresStoreBuilder, KeyvError};
/// # use std::sync::Arc;
/// # use sqlx::PgPool;
/// # #[tokio::main]
/// # async fn main() -> Result<(), KeyvError> {
/// let store = PostgresStoreBuilder::new()
///     .uri("postgres://username:password@localhost/database")
///     .table_name("custom_table_name")
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// ## Using an Existing Connection Pool
///
/// ```no_run
/// # use your_crate::{PostgresStoreBuilder, KeyvError};
/// # use std::sync::Arc;
/// # use sqlx::{PgPool, PgPoolOptions};
/// # #[tokio::main]
/// # async fn main() -> Result<(), KeyvError> {
/// let pool: Arc<PgPool> = Arc::new(PgPoolOptions::new()
///     .connect("postgres://username:password@localhost/database")
///     .await?);
///
/// let store = PostgresStoreBuilder::new()
///     .pool(pool)
///     .table_name("custom_table_name")
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct PostgresStoreBuilder {
    uri: Option<String>,
    pool: Option<Arc<PgPool>>,
    table_name: String,
}

/// Creates a new builder instance with default configuration.
///
/// Initializes the builder with the default table name and no predefined URI or connection pool.
/// The default table name is defined by `DEFAULT_TABLE_NAME`.
impl PostgresStoreBuilder {
    pub fn new() -> Self {
        Self {
            uri: None,
            pool: None,
            table_name: DEFAUTL_TABLE_NAME.to_string(),
        }
    }

    /// Sets the table name for the `PostgresStore`.
    ///
    /// This method configures the table name to be used by the store. If not set,
    /// `DEFAULT_TABLE_NAME` will be used.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table used to store key-value pairs.
    pub fn table_name<S: Into<String>>(mut self, table: S) -> Self {
        self.table_name = table.into();
        self
    }
    /// Sets the database URI for connecting to the PostgreSQL database.
    ///
    /// This method configures the database URI. It's required if no existing connection pool is provided.
    ///
    /// # Arguments
    ///
    /// * `uri` - The database URI string.
    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Uses an existing connection pool for the `PostgresStore`.
    ///
    /// This method allows for using an already configured `PgPool`. If set,
    /// the `uri` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `pool` - Shared reference to an existing `PgPool`.
    pub fn pool(mut self, pool: Arc<PgPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Builds the `PostgresStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates a `PostgresStore` instance.
    /// It requires either a database URI or an existing connection pool to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized `PostgresStore`.
    /// On failure, it returns a `KeyvError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<PostgresStore, StoreError> {
        let pool = match self.pool {
            Some(pool) => pool,
            None => {
                let uri = self
                    .uri
                    .expect("PostgresStore requires either a URI or an existing pool to be set");
                Arc::new(
                    PgPoolOptions::new()
                        .connect(&uri)
                        .await
                        .map_err(|e| StoreError::ConnectionError(e.to_string()))?,
                )
            }
        };

        Ok(PostgresStore {
            pool,
            table_name: self.table_name,
        })
    }
}

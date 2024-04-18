use std::sync::Arc;

use mongodb::{options::ClientOptions, Client};

use crate::{StoreError, DEFAUTL_NAMESPACE_NAME};

use super::MongoStore;

/// Builder for creating a `MongoStore`.
///
/// This builder enables configuring a `MongoStore` with custom
/// settings such as a MongoDB URI, a specific database and collection name, or using an existing client instance.
/// It provides flexibility for setting up the store based on the needs of the application.
///
/// # Examples
///
/// ## Initializing with a MongoDB URI
///
/// ```rust,no_run
/// # use keyv::adapter::mongodb::{MongoStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let store = MongoStoreBuilder::new()
///     .uri("mongodb://username:password@localhost")
///     .database_name("custom_database")
///     .collection_name("custom_collection")
///     .build()
///     .await.unwrap();
/// }
/// ```
///
/// ## Using an Existing Client
///
/// ```rust,no_run
/// # use mongodb::{Client, options::ClientOptions};
/// # use std::sync::Arc;
/// # use keyv::adapter::mongodb::{MongoStoreBuilder};
/// # #[tokio::main]
/// # async fn main() {
/// let client_options = ClientOptions::parse("mongodb://username:password@localhost").await.unwrap();
/// let client = Client::with_options(client_options).unwrap();
/// let store = MongoStoreBuilder::new()
///     .client(Arc::new(client))
///     .database_name("custom_database")
///     .collection_name("custom_collection")
///     .build()
///     .await.unwrap();
/// }
/// ```
pub struct MongoStoreBuilder {
    uri: Option<String>,
    database_name: Option<String>,
    collection_name: Option<String>,
    client: Option<Arc<Client>>,
}

impl MongoStoreBuilder {
    /// Creates a new builder instance with default configuration.
    ///
    /// Initializes the builder with no predefined URI, database, or collection name,
    /// allowing these to be set according to specific requirements.
    pub fn new() -> Self {
        Self {
            uri: None,
            database_name: None,
            collection_name: None,
            client: None,
        }
    }

    /// Sets the MongoDB URI for connecting to the database.
    ///
    /// This method configures the MongoDB URI. It's required if no existing client is used.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI string used to connect to MongoDB.
    pub fn uri<S: Into<String>>(mut self, uri: S) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Sets the database name for the `MongoStore`.
    ///
    /// This method selects the database within MongoDB to be used. If not set, a default name is used.
    ///
    /// # Arguments
    ///
    /// * `database_name` - The name of the database.
    pub fn database_name<S: Into<String>>(mut self, database_name: S) -> Self {
        self.database_name = Some(database_name.into());
        self
    }

    /// Sets the collection name for the `MongoStore`.
    ///
    /// This method selects the collection within the database to be used. If not set, a default name is used.
    ///
    /// # Arguments
    ///
    /// * `collection_name` - The name of the collection.
    pub fn collection_name<S: Into<String>>(mut self, collection_name: S) -> Self {
        self.collection_name = Some(collection_name.into());
        self
    }

    /// Uses an existing client for the `MongoStore`.
    ///
    /// This method allows for using an already configured MongoDB `Client`. If set,
    /// the `uri` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `client` - Shared reference to an existing `Client`.
    pub fn client(mut self, client: Arc<Client>) -> Self {
        self.client = Some(client);
        self
    }

    /// Builds the `MongoStore` based on the provided configurations.
    ///
    /// Finalizes the builder and creates a `MongoStore` instance.
    /// It requires either a MongoDB URI or an existing client to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized `MongoStore`.
    /// On failure, it returns a `StoreError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<MongoStore, StoreError> {
        let client = match self.client {
            Some(client) => client,
            None => {
                let uri = self
                    .uri
                    .expect("MongoDB requires a URI or an existing client to be set");

                let options = ClientOptions::parse(&uri)
                    .await
                    .map_err(|e| StoreError::ConnectionError(e.to_string()))?;
                Arc::new(
                    Client::with_options(options)
                        .map_err(|e| StoreError::ConnectionError(e.to_string()))?,
                )
            }
        };

        let database_name = match &self.database_name {
            Some(db_name) => db_name.to_string(),
            None => {
                log::warn!("Database name not provided, using default");
                DEFAUTL_NAMESPACE_NAME.to_string()
            }
        };

        let collection_name = match &self.collection_name {
            Some(coll_name) => coll_name.to_string(),
            None => {
                log::warn!("Collection name not provided, using default");
                DEFAUTL_NAMESPACE_NAME.to_string()
            }
        };

        Ok(MongoStore {
            client,
            database_name,
            collection_name,
        })
    }
}

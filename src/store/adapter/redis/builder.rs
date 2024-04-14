use std::sync::Arc;

use redis::Client;

use crate::StoreError;

use super::RedisStore;

pub struct RedisStoreBuilder {
    connection_string: Option<String>,
    client: Option<Arc<Client>>,
    default_ttl: Option<u64>,
}

/// Builder for creating a `RedisStore`.
///
/// This builder allows for configuring a `RedisStore` with custom
/// settings such as a specific connection string, an existing client,
/// and a default TTL (time to live) for the keys. It provides a flexible
/// way to initialize the store depending on the application's requirements
///
/// # Examples
///
/// ## Initializing with a Connection String
///
/// This example demonstrates how to create a `RedisStore` using a connection string.
///
/// ```rust
/// # use keyv::adapter::redis::{RedisStoreBuilder};
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// let store = RedisStoreBuilder::new()
///     .uri("redis://username:password@localhost:6379")
///     .default_ttl(3600) // Sets the default TTL for keys to 3600 seconds (1 hour)
///     .build()
///     .await
///     .unwrap();
/// # }
/// ```
///
/// ## Using an Existing Client
///
/// This example shows how to initialize a `RedisStore` with an existing Redis client instance.
///
/// ```rust
/// # use keyv::adapter::redis::{RedisStoreBuilder};
/// # use redis::Client;
/// # use std::sync::Arc;
/// # #[tokio::main]
/// # async fn main() {
/// # let client_url = "redis://username:password@localhost:6379";
/// let client: Arc<Client> = Arc::new(Client::open(client_url).unwrap());
///
/// let store = RedisStoreBuilder::new()
///     .client(client)
///     .default_ttl(7200) // Sets the default TTL for keys to 7200 seconds (2 hours)
///     .build()
///     .await
///     .unwrap();
/// # }
/// ```
///
/// These examples demonstrate the versatility of `RedisStoreBuilder` in configuring a `RedisStore`, either through direct connection parameters or by leveraging an existing client setup.
impl RedisStoreBuilder {
    /// Creates a new builder instance with default configuration.
    ///
    /// Initializes the builder with no predefined connection string or client,
    /// and no default TTL for the keys.
    pub fn new() -> Self {
        Self {
            connection_string: None,
            client: None,
            default_ttl: None,
        }
    }

    /// Sets the connection string for connecting to the Redis database.
    ///
    /// This method configures the Redis connection string. It's required if no existing client is provided.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The Redis connection string.
    pub fn uri<S: Into<String>>(mut self, connection_string: S) -> Self {
        self.connection_string = Some(connection_string.into());
        self
    }

    /// Uses an existing client for the `RedisStore`.
    ///
    /// This method allows for using an already configured `Client`. If set,
    /// the `connection_string` option is ignored.
    ///
    /// # Arguments
    ///
    /// * `client` - A shared reference to an existing `Client`.
    pub fn client(mut self, client: Arc<Client>) -> Self {
        self.client = Some(client);
        self
    }

    /// Sets the default TTL (time to live) for the keys in the `RedisStore`.
    ///
    /// This method configures the default TTL for the keys. If not set,
    /// keys will not expire by default.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The time to live in seconds.
    pub fn default_ttl(mut self, ttl: u64) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    /// Builds the `RedisStore` based on the provided configurations.
    ///
    /// Finalizes the builder process and creates a `RedisStore` instance.
    /// It requires either a connection string or an existing client to be set.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` which, on success, contains the initialized `RedisStore`.
    /// On failure, it returns a `StoreError` indicating what went wrong during the initialization.
    pub async fn build(self) -> Result<RedisStore, StoreError> {
        let client = match self.client {
            Some(client) => client,
            None => {
                let connection_string = self
                    .connection_string
                    .expect("A connection string or an existing client must be set");
                Arc::new(
                    Client::open(connection_string)
                        .map_err(|e| StoreError::ConnectionError(e.to_string()))?,
                )
            }
        };

        Ok(RedisStore {
            client,
            default_ttl: self.default_ttl,
        })
    }
}

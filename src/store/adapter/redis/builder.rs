use std::sync::Arc;

use redis::Client;

use crate::StoreError;

use super::RedisStore;

pub struct RedisStoreBuilder {
    connection_string: Option<String>,
    client: Option<Arc<Client>>,
    default_ttl: Option<u64>,
}

impl RedisStoreBuilder {
    pub fn new() -> Self {
        Self {
            connection_string: None,
            client: None,
            default_ttl: None,
        }
    }

    pub fn uri<S: Into<String>>(mut self, connection_string: S) -> Self {
        self.connection_string = Some(connection_string.into());
        self
    }

    pub fn client(mut self, client: Arc<Client>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn default_ttl(mut self, ttl: u64) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

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

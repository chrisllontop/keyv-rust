use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Failed to connect to the database backend: {0}")]
    ConnectionError(String),

    #[error("Error while serializing or deserializing data")]
    SerializationError {
        #[from]
        source: serde_json::Error,
    },

    #[error("Database operation failed")]
    DatabaseError {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Database query error: {0}")]
    QueryError(String),

    #[error("The requested key was not found")]
    NotFound,

    #[error("An unknown error has occurred")]
    Unknown,
}

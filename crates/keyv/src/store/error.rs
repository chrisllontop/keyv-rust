use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyvError {
    #[error("Failed to connect to the database backend")]
    ConnectionError,

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

    #[error("The requested key was not found")]
    NotFound,

    #[error("An unknown error has occurred")]
    Unknown,
}

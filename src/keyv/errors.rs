use thiserror::Error;

use crate::store::StoreError;

#[derive(Error, Debug)]
pub enum KeyvError {
    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),
}

use thiserror::Error;

// Wrapped result for convienence
pub type ClientResult<T> = std::result::Result<T, ClientError>;

#[derive(Clone, Debug, Error)]
pub enum ClientError {
    #[error("Could not find transaction by id of {0}")]
    TransactionNotFound(u16),
}
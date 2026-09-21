use thiserror::Error;

mod account_error;
mod client_error;

pub use account_error::*;
pub use client_error::*;

// Wrapped result for convienence
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Environmental Variable {0} is not set")]
    EnvVariable(String),
    #[error("Missing argument for {0}")]
    MissingArgument(String),

    #[error("Csv Error: {0}")]
    Csv(String),

    #[error("Failed to parse {0} from {1}")]
    Parse(&'static str, String),

    #[error("Account error: {0}")]
    Account(#[from] AccountError),

    #[error("Client error: {0}")]
    Client(#[from] ClientError),
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error.to_string())
    }
}

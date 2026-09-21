use thiserror::Error;

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
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error.to_string())
    }
}

pub mod constants;
pub mod error;
pub mod models;

use std::{fs::File, path::PathBuf};

pub use constants::*;
use csv::Trim;
pub use error::*;
pub use models::*;

pub async fn true_main(args: Vec<String>) -> Result<System> {
    let path = match args.get(1) {
        Some(name) => {
            let mut path = PathBuf::new();
            path.push(name.clone());
            path
        }
        #[cfg(debug_assertions)]
        None => std::env::var("INPUT_FILE")
            .map(PathBuf::from)
            .map_err(|_| Error::MissingArgument("INPUT_FILE".to_string()))?,

        #[cfg(not(debug_assertions))]
        None => return Err(Error::MissingArgument("INPUT_FILE".to_string())),
    };

    tracing::error!("Reading file from {path:?}: {}", path.exists());

    let mut reader_builder = csv::ReaderBuilder::new();
    reader_builder.trim(Trim::All);
    reader_builder.has_headers(true);
    let reader = reader_builder.from_path(path)?;

    let mut system = System::new();

    system.ingest(reader).await?;

    Ok(system)
}

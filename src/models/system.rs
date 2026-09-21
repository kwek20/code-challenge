use std::fs::File;

use csv::Reader;
use serde::{Deserialize, Serialize};

use crate::{Account, Client, Result};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct System {
    client: Vec<Client>,
    accounts: Vec<Account>,
}

impl System {
    pub fn new() -> Self {
        Self {
            client: vec![],
            accounts: vec![],
        }
    }

    pub async fn ingest(&self, reader: Reader<File>) -> Result<()> {
        Ok(())
    }
}

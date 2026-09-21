use serde::{Deserialize, Serialize};

use crate::Account;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Client {
    client_id: u16,
    account: Account,
}

impl Client {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            account: Account::new(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {}

impl Account {
    pub fn new() -> Self {
        Self {}
    }
}

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemOutput {
    pub client: u16,
    pub available: u16,
    pub held: u16,
    pub total: u16,
    pub locked: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TransactionRecord {
    pub transaction_type: Transaction,
    pub client: u16,
    pub tx: u16,
    pub amount: u16
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Transaction {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl FromStr for Transaction {
    type Err = Error;

    fn from_str(transaction: &str) -> Result<Self, Self::Err> {
        match transaction {
            "deposit" => Ok(Self::Deposit),
            "withdrawal" => Ok(Self::Withdrawal),
            "dispute" => Ok(Self::Dispute),
            "resolve" => Ok(Self::Resolve),
            "chargeback" => Ok(Self::Chargeback),
            _ => Err(Error::Parse("Transaction", transaction.to_string())),
        }
    }
}

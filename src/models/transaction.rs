use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ClientOutput {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TransactionRecord {
    pub r#type: Transaction,
    pub client: u16,
    pub tx: u32,
    pub amount: Decimal
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UserTransaction {
    pub transaction_type: Transaction,
    pub tx: u32,
    pub amount: Decimal
}

impl From<TransactionRecord> for UserTransaction {
    fn from(record: TransactionRecord) -> Self {
        Self {
            transaction_type: record.r#type,
            tx: record.tx,
            amount: record.amount,
        }
    }
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

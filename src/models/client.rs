use serde::{Deserialize, Serialize};

use crate::{Account, Result, Transaction, TransactionRecord};

pub type ClientId = u16;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Client {
    client_id: ClientId,
    account: Account,
}

impl Client {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            account: Account::new(),
        }
    }

    pub async fn process(&mut self, record: &TransactionRecord) -> Result<()> {
        match &record.transaction_type {
            Transaction::Deposit => self.process_deposit(record).await,
            Transaction::Withdrawal => self.process_withdrawal(record).await,
            Transaction::Dispute => self.process_dispute(record).await,
            Transaction::Resolve => self.process_resolve(record).await,
            Transaction::Chargeback => self.process_chargeback(record).await,
        }
    }

    pub async fn process_deposit(&mut self, _record: &TransactionRecord) -> Result<()> {
        todo!()
    }

    pub async fn process_withdrawal(&mut self, _record: &TransactionRecord) -> Result<()> {
        todo!()
    }

    pub async fn process_dispute(&mut self, _record: &TransactionRecord) -> Result<()> {
        todo!()
    }

    pub async fn process_resolve(&mut self, _record: &TransactionRecord) -> Result<()> {
        todo!()
    }

    pub async fn process_chargeback(&mut self, _record: &TransactionRecord) -> Result<()> {
        todo!()
    }
}

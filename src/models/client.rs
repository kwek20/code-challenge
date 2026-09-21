use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{Account, AccountError, ClientError, ClientOutput, Result, Transaction, TransactionRecord, UserTransaction};

pub type ClientId = u16;

fn transactions() -> Arc<RwLock<Vec<UserTransaction>>> {
    Arc::new(RwLock::new(vec![]))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    client_id: ClientId,
    account: Account,
    #[serde(skip, default = "transactions")]
    transactions: Arc<RwLock<Vec<UserTransaction>>>
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.client_id == other.client_id
            && self.account == other.account
    }
}

impl Eq for Client {}

impl Client {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            account: Account::new(),
            transactions: transactions()
        }
    }

    pub fn id(&self) -> ClientId {
        self.client_id
    }

    pub fn record(&self) -> ClientOutput {
        ClientOutput {
            client: self.client_id,
            available: self.account.available(),
            held: self.account.held(),
            total: self.account.total(),
            locked: !self.account.may_modify(),
        }
    }

    pub async fn process(&mut self, record: TransactionRecord) -> Result<()> {
        let result = match &record.r#type {
            Transaction::Deposit => self.process_deposit(&record).await,
            Transaction::Withdrawal => self.process_withdrawal(&record).await,
            Transaction::Dispute => self.process_dispute(&record).await,
            Transaction::Resolve => self.process_resolve(&record).await,
            Transaction::Chargeback => self.process_chargeback(&record).await,
        };

        if let Err(e) = &result {
            tracing::error!("{}: Failed to process transaction {} due to {e}", record.client, record.tx)
        } else {
            self.transactions.write().await.push(record.into());
        }

        Ok(())
    }

    pub async fn process_deposit(&mut self, record: &TransactionRecord) -> Result<()> {
        self.account.add(record.amount).await
    }

    pub async fn process_withdrawal(&mut self, record: &TransactionRecord) -> Result<()> {
        if !self.account.remove(record.amount).await? {
            return Err(AccountError::WithdrawalFailed)?
        }

        Ok(())
    }

    pub async fn process_dispute(&mut self, record: &TransactionRecord) -> Result<()> {
        let dispute = record.tx;
        let disputed_tx = self.get_transaction(record.tx).await;

        match disputed_tx {
            None => Err(ClientError::TransactionNotFound(dispute))?,
            Some(t) => self.account.hold(t.amount).await
        }
    }

    pub async fn process_resolve(&mut self, record: &TransactionRecord) -> Result<()> {
        let dispute = record.tx;
        if !self.is_disputed(dispute).await {
            return Err(ClientError::NotDisputed(dispute))?;
        }
        
        let disputed_tx = self.get_transaction(record.tx).await;
    
        match disputed_tx {
            None => Err(ClientError::TransactionNotFound(dispute))?,
            Some(t) => self.account.make_available(t.amount).await
        }
    }

    pub async fn process_chargeback(&mut self, record: &TransactionRecord) -> Result<()> {
        let dispute = record.tx;
        if !self.is_disputed(dispute).await {
            return Err(ClientError::NotDisputed(dispute))?;
        }
        
        let disputed_tx = self.get_transaction(record.tx).await;
    
        match disputed_tx {
            None => Err(ClientError::TransactionNotFound(dispute))?,
            Some(t) => self.account.chargeback(t.amount).await?
        }

        // Lock the account after a chargeback
        self.account.lock().await;

       Ok(())
    }

    async fn get_transaction(&self, tx_id: u32) -> Option<UserTransaction> {
        let lock = self.transactions.read().await;
        lock.iter().find(|t| t.tx == tx_id).cloned()
    }

    /// Check if a certain transaciton id is currently disputed
    async fn is_disputed(&self, tx_id: u32) ->  bool {
        let mut disputed = false;
        self.transactions.read().await.iter().for_each(|t| {
            if t.tx != tx_id {
                return;
            }

            match t.transaction_type {
                Transaction::Dispute => disputed = true,
                Transaction::Resolve => disputed = false,
                _ => {}, // Ignore the rest
            };
        });

        disputed
    }
}

use std::sync::Arc;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use crate::{AccountError, AccountResult, Result};

fn semaphore() -> Arc<Semaphore> {
    Arc::new(Semaphore::new(1))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    /// This should be equal to the total - held amounts
    available: Decimal,
    /// The total funds that are held for dispute
    held: Decimal,
    /// The total funds that are available or held
    total: Decimal,
    /// Whether the account is locked
    locked: bool,
    /// Prevent concurrent writing
    #[serde(skip, default = "semaphore")]
    lock: Arc<Semaphore>,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.available == other.available
            && self.held == other.held
            && self.total == other.total
            && self.locked == other.locked
    }
}

impl Eq for Account {}

impl Default for Account {
    fn default() -> Self {
        Self {
            lock: semaphore(),
            available: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
        }
    }
}

impl Account {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the account balances can be modified
    pub fn may_modify(&self) -> bool {
        !self.locked
    }

    pub fn assert_modify(&self) -> AccountResult<()> {
        match self.may_modify() {
            true => Ok(()),
            false => Err(AccountError::ModificationsLocked),
        }
    }

    pub async fn lock(&mut self) {
        let _ = self.lock.acquire().await;
        self.locked = true;
    }

    pub async fn unlock(&mut self) {
        let _ = self.lock.acquire().await;
        self.locked = false;
    }

    pub fn available(&self) -> Decimal {
        self.available
    }

    pub fn held(&self) -> Decimal {
        self.held
    }

    pub fn total(&self) -> Decimal {
        self.total
    }

    /// Add funds to the account
    pub async fn add(&mut self, amount: Decimal) -> Result<()> {
        let _ = self.lock.acquire().await;
        self.assert_modify()?;

        self.available += amount;
        self.total += amount;

        Ok(())
    }

    /// Remove funds from the account
    pub async fn remove(&mut self, amount: Decimal) -> Result<bool> {
        let _ = self.lock.acquire().await;

        self.assert_modify()?;

        if self.available < amount {
            return Ok(false);
        }

        self.available -= amount;
        self.total -= amount;

        Ok(true)
    }

    /// Hold a certain amount of funds
    pub async fn hold(&mut self, amount: Decimal) -> Result<()> {
        let _ = self.lock.acquire().await;
        self.assert_modify()?;

        self.available -= amount;
        self.held += amount;

        Ok(())
    }

    /// Make a certain amount of held funds available again
    pub async fn make_available(&mut self, amount: Decimal) -> Result<()> {
        let _ = self.lock.acquire().await;
        self.assert_modify()?;

        self.available += amount;
        self.held -= amount;

        Ok(())
    }

    /// Remove an amount from the user, which were initially held back
    pub async fn chargeback(&mut self, amount: Decimal) -> Result<()> {
        let _ = self.lock.acquire().await;
        self.assert_modify()?;

        self.total -= amount;
        self.held -= amount;

        Ok(())
    }
}

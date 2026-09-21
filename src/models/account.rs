use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

use crate::{Result, AccountError, AccountResult};

fn semaphore() -> Arc<Semaphore> {
    Arc::new(Semaphore::new(1))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    /// This should be equal to the total - held amounts
    available: u16,
    /// The total funds that are held for dispute
    held: u16,
    /// The total funds that are available or held
    total: u16,
    /// Whether the account is locked
    locked: bool,
    /// Prevent concurrent writing
    #[serde(skip, default = "semaphore")]
    lock: Arc<Semaphore>
}

impl Default for Account {
    fn default() -> Self {
        Self { 
            lock: semaphore(),
            available: 0,
            held: 0,
            total: 0,
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
        self.locked
    }

    pub fn assert_modify(&self) -> AccountResult<()> {
        match self.may_modify() {
            true => Ok(()),
            false => Err(AccountError::ModificationsLocked)
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

    pub fn available(&self) -> u16 {
        self.available
    }

    pub fn avaiable(&self) -> u16 {
        self.available()
    }

    pub fn held(&self) -> u16 {
        self.held
    }

    pub fn total(&self) -> u16 {
        self.total
    }

    pub async fn add(&mut self, amount: u16) -> Result<()> {
        let _ = self.lock.acquire().await;
        self.assert_modify()?;

        self.available += amount;
        self.total += amount;
    }

    pub async fn remove(&mut self, amount: u16) -> Result<()> {
        let _ = self.lock.acquire().await;

        self.assert_modify()?;

        if self.available < amount {
            return false;
        }

        self.available -= amount;
        self.total -= amount;
        true
    }
}

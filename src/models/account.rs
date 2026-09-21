use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// This should be equal to the total - held amounts
    available: u16,
    /// The total funds that are held for dispute
    held: u16,
    /// The total funds that are available or held
    total: u16,
    /// Whether the account is locked
    locked: bool
}

impl Account {
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the account balances can be modified
    pub fn may_modify(&self) -> bool {
        self.locked
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
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

    pub fn add(&mut self, amount: u16) {
        self.available += amount;
        self.total += amount;
    }

    pub fn remove(&mut self, amount: u16) -> bool {
        if self.available < amount {
            return false;
        }

        self.available -= amount;
        self.total -= amount;
        true
    }
}

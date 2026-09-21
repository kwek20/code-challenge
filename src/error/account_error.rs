use thiserror::Error;

// Wrapped result for convienence
pub type AccountResult<T> = std::result::Result<T, AccountError>;

#[derive(Clone, Debug, Error)]
pub enum AccountError {
    #[error("Account may not be modified at this time")]
    ModificationsLocked,
    #[error("Account could not withdraw funds at this time")]
    WithdrawalFailed,
}

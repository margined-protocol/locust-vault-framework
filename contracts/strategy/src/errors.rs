use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unable to perform migration")]
    MigrationError {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Vault not set")]
    VaultNotSet {},

    #[error("Send authorization not set")]
    SendAuthorizationNotSet {},
}

impl ContractError {
    pub fn generic_err(msg: impl Into<String>) -> ContractError {
        ContractError::Std(StdError::generic_err(msg))
    }
}

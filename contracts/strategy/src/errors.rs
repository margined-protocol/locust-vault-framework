use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Unable to perform migration")]
    MigrationError {},

    #[error("Non-payable entry point")]
    NonPayable {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Vault not set")]
    VaultNotSet {},
}

impl ContractError {
    pub fn generic_err(msg: impl Into<String>) -> ContractError {
        ContractError::Std(StdError::generic_err(msg))
    }
}

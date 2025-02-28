use crate::errors::ContractError;

use cosmwasm_std::{Coin, StdError};
use std::fmt::Display;

pub fn tokens_to_string(tokens: Vec<Coin>) -> String {
    tokens
        .iter()
        .map(|coin| format!("{}{}", coin.amount, coin.denom))
        .collect::<Vec<String>>()
        .join(",")
}

pub fn map_to_contract_error<E: Display>(e: E) -> ContractError {
    ContractError::Std(StdError::generic_err(e.to_string()))
}

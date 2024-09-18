use crate::{errors::ContractError, state::CONFIG};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure, to_json_binary, Coin, DepsMut, Env, MessageInfo, Response, StdError, WasmMsg,
};
use cw_utils::nonpayable;

#[cw_serde]
pub enum VaultMsg {
    Withdraw { tokens_to_withdraw: Vec<Coin> },
    Repay {},
}

pub fn handle_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tokens_to_withdraw: Vec<Coin>,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))?;

    let config = CONFIG.load(deps.storage)?;

    ensure!(
        config.controller == info.sender,
        ContractError::Unauthorized {}
    );

    let msg = WasmMsg::Execute {
        msg: to_json_binary(&VaultMsg::Withdraw { tokens_to_withdraw })?,
        funds: vec![],
        contract_addr: config.vault.to_string(),
    };

    Ok(Response::default().add_message(msg))
}

pub fn handle_repay(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tokens_to_repay: Vec<Coin>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    ensure!(
        config.controller == info.sender,
        ContractError::Unauthorized {}
    );

    let msg = WasmMsg::Execute {
        msg: to_json_binary(&VaultMsg::Repay {})?,
        funds: tokens_to_repay,
        contract_addr: config.vault.to_string(),
    };

    Ok(Response::default().add_message(msg))
}

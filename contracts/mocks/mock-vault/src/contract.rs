use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_vault_standard::VaultStandardInfoResponse;

#[cw_serde]
pub struct InstantiateMsg {}

use cw_vault_standard::VaultStandardExecuteMsg;

pub type VaultExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

#[cw_serde]
pub enum ExtensionExecuteMsg {
    Vaultenator(VaultenatorExtensionExecuteMsg),
}

#[cw_serde]
pub enum VaultenatorExtensionExecuteMsg {
    Withdraw { tokens_to_withdraw: Vec<Coin> },
    Repay {},
}

#[cw_serde]
pub enum QueryMsg {}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Withdraw { tokens_to_withdraw } => withdraw(deps, info, tokens_to_withdraw),
        ExecuteMsg::Repay {} => repay(deps, info),
    }
}

#[cfg(not(tarpaulin_include))]
pub fn withdraw(
    _deps: DepsMut,
    info: MessageInfo,
    tokens_to_withdraw: Vec<Coin>,
) -> StdResult<Response> {
    let mut response = Response::default();

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: tokens_to_withdraw,
    };

    response = response.add_message(msg);

    Ok(response)
}

#[cfg(not(tarpaulin_include))]
pub fn repay(_deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

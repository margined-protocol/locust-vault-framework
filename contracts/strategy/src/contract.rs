use crate::{
    errors::ContractError,
    handle::{handle_repay, handle_set_grants, handle_set_vault, handle_withdraw},
    query::{query_config, query_grants, query_spot_price, query_twap_price},
    state::{Config, CONFIG},
    utils::create_authz_grant_messages,
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use interface::strategy::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use std::iter::Iterator;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;

    let config = Config {
        admin: msg.admin,
        controller: msg.controller,
        vault: None,
        token0: msg.token0,
        token1: msg.token1,
        grants: msg.grants,
        pool_info: msg.pool_info,
    };

    config.validate(&deps.as_ref())?;

    CONFIG.save(deps.storage, &config)?;

    let grantee = config.controller;
    let grants: Vec<&str> = config.grants.iter().map(|s| s.as_str()).collect();

    let authz_msgs = create_authz_grant_messages(env.contract.address.as_str(), &grantee, &grants);

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_messages(authz_msgs))
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw { tokens_to_withdraw } => {
            handle_withdraw(deps, env, info, tokens_to_withdraw)
        }
        ExecuteMsg::Repay { tokens_to_repay } => handle_repay(deps, env, info, tokens_to_repay),
        ExecuteMsg::SetVault { vault } => handle_set_vault(deps, env, info, vault),
        ExecuteMsg::SetGrants { grants } => handle_set_grants(deps, env, info, grants),
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(&deps)?),
        QueryMsg::Grants {} => to_json_binary(&query_grants(&deps)?),
        QueryMsg::SpotPrice {} => to_json_binary(&query_spot_price(&deps)?),
        QueryMsg::TwapPrice { duration } => {
            to_json_binary(&query_twap_price(&deps, env, duration)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    unimplemented!()
}

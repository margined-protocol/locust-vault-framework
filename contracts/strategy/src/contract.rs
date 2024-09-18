use crate::{
    errors::ContractError,
    handle::{handle_repay, handle_withdraw},
    query::{query_config, query_grants, query_spot_price, query_twap_price},
    state::{Config, CONFIG},
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use interface::strategy::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        format!("crates.io:{CONTRACT_NAME}"),
        CONTRACT_VERSION,
    )?;

    let config = Config {
        controller: msg.controller,
        vault: msg.vault,
        token0: msg.token0,
        token1: msg.token1,
        grants: msg.grants,
        pool_info: msg.pool_info,
    };

    config.validate(&deps.as_ref())?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
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

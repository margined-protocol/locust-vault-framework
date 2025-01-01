use crate::{
    errors::ContractError,
    events::event_migrate,
    handle::{
        handle_repay, handle_repay_queue, handle_set_grants, handle_set_vault,
        handle_update_config, handle_withdraw,
    },
    ownership::{
        get_ownership_proposal, handle_claim_ownership, handle_ownership_proposal,
        handle_ownership_proposal_rejection,
    },
    query::{query_config, query_grants, query_owner, query_spot_price, query_twap_price},
    state::{Config, CONFIG, OWNER, OWNERSHIP_PROPOSAL},
    utils::create_authz_grant_messages,
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use interface::strategy::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use std::iter::Iterator;

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
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

    OWNER.set(deps, Some(info.sender.clone()))?;

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
        ExecuteMsg::Repay {
            tokens_to_repay,
            cycle_profit,
        } => handle_repay(deps, env, info, tokens_to_repay, cycle_profit),
        ExecuteMsg::RepayQueue {
            tokens_to_repay,
            cycle_profit,
            limit,
        } => handle_repay_queue(deps, env, info, tokens_to_repay, cycle_profit, limit),
        ExecuteMsg::SetVault { vault } => handle_set_vault(deps, env, info, vault),
        ExecuteMsg::SetGrants { grants } => handle_set_grants(deps, env, info, grants),
        ExecuteMsg::UpdateConfig { grants, controller } => {
            handle_update_config(deps, env, info, grants, controller)
        }
        ExecuteMsg::ProposeNewOwner {
            new_owner,
            duration,
        } => handle_ownership_proposal(
            deps,
            info,
            env,
            new_owner,
            duration,
            OWNER,
            OWNERSHIP_PROPOSAL,
        ),
        ExecuteMsg::RejectOwner {} => {
            handle_ownership_proposal_rejection(deps, info, OWNER, OWNERSHIP_PROPOSAL)
        }
        ExecuteMsg::ClaimOwnership {} => {
            handle_claim_ownership(deps, info, env, OWNER, OWNERSHIP_PROPOSAL)
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(&deps)?),
        QueryMsg::Grants {} => to_json_binary(&query_grants(&deps)?),
        QueryMsg::SpotPrice {} => to_json_binary(&query_spot_price(&deps, env)?),
        QueryMsg::TwapPrice { duration } => {
            to_json_binary(&query_twap_price(&deps, env, duration)?)
        }
        QueryMsg::Owner {} => to_json_binary(
            &query_owner(deps).map_err(|err| StdError::generic_err(err.to_string()))?,
        ),
        QueryMsg::GetOwnershipProposal {} => {
            to_json_binary(&get_ownership_proposal(deps, OWNERSHIP_PROPOSAL)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;

    match contract_version.contract.as_ref() {
        "crates.io:strategy" => match contract_version.version.as_ref() {
            "0.0.4" => {
                set_contract_version(
                    deps.storage,
                    format!("crates.io:{CONTRACT_NAME}"),
                    CONTRACT_VERSION,
                )?;
            }
            _ => {
                return Err(ContractError::Std(StdError::generic_err(
                    "Migration failed",
                )))
            }
        },
        _ => {
            return Err(ContractError::Std(StdError::generic_err(
                "Migration failed",
            )))
        }
    }

    Ok(Response::new().add_event(event_migrate(
        CONTRACT_VERSION,
        CONTRACT_NAME,
        contract_version,
    )))
}

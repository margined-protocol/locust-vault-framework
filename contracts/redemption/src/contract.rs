use crate::{
    errors::ContractError,
    handle::{handle_claim_redemption, handle_send_redemption, handle_update_config},
    ownership::{
        get_ownership_proposal, handle_claim_ownership, handle_ownership_proposal,
        handle_ownership_proposal_rejection,
    },
    query::{query_all_redemptions, query_config, query_owner, query_redemptions},
    storage::state::{Config, CONFIG, OWNER, OWNERSHIP_PROPOSAL},
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw2::set_contract_version;
use interface::redemption::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
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
        whitelisted_funds: msg.whitelisted_funds,
    };

    config.validate(&deps.as_ref())?;

    CONFIG.save(deps.storage, &config)?;

    OWNER.set(deps, Some(info.sender.clone()))?;

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
        ExecuteMsg::SendRedemption { redemption } => {
            handle_send_redemption(deps, env, info, redemption)
        }
        ExecuteMsg::ClaimRedemption { limit } => handle_claim_redemption(deps, env, info, limit),
        ExecuteMsg::UpdateConfig {
            add_fund,
            remove_fund,
        } => handle_update_config(deps, env, info, add_fund, remove_fund),
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(&deps)?),
        QueryMsg::AllRedemptions { start_after, limit } => {
            to_json_binary(&query_all_redemptions(deps, start_after, limit)?)
        }
        QueryMsg::Redemptions { user, limit } => {
            to_json_binary(&query_redemptions(deps, user, limit)?)
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
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    unimplemented!()
}

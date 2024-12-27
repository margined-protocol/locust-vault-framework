use crate::{
    handlers::extensions::{
        handle_cancel_redemption, handle_create_redemption, handle_repay, handle_repay_queue,
        handle_withdraw,
    },
    queries::extensions::{
        query_estimate_vault_assets, query_pending_redemptions, query_state_wrapper,
        query_user_redemption, query_version, query_withdrawable_amount,
    },
    storage::{config::Config, state::State},
    sudo::{handle_register_sudo, sudo_block_before_send},
};

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult,
};
use cw_vault_standard::VaultStandardInfoResponse;
use interface::fund::{
    ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, InstantiateMsg, MigrateMsg, QueryMsg,
    SudoMsg, VaultenatorExtensionExecuteMsg, VaultenatorExtensionQueryMsg,
};
use vaultenator::{
    admin::Administer,
    config::Configure,
    contract::{Describe, Vaultenator},
    errors::ContractError,
    handlers::Handle,
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
    state::{OWNER, OWNERSHIP_PROPOSAL},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct StructuredVault;

// # Custom trait implementations
//
// - Configure implemented in src/config.rs.
// - Describe implemented in src/describe.rs
// - Handle implemented in src/handle.rs.
// - Query implemented in src/query.rs.
// - ManageState implemented in src/state.rs.
// - ReplyHandler implemented in src/reply.rs.

impl Own for StructuredVault {}
impl Administer<State> for StructuredVault {}
impl Vaultenator<Config, State> for StructuredVault {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    StructuredVault.handle_instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        #[allow(deprecated)]
        ExecuteMsg::Deposit { amount, recipient } => {
            StructuredVault.handle_deposit(deps, env, info, amount, recipient)
        }
        #[allow(deprecated)]
        ExecuteMsg::Redeem { recipient, amount } => {
            StructuredVault.handle_redeem(deps, env, info, amount, recipient)
        }
        ExecuteMsg::VaultExtension(msg) => match msg {
            ExtensionExecuteMsg::Vaultenator(msg) => match msg {
                VaultenatorExtensionExecuteMsg::CancelRedemption {} => {
                    handle_cancel_redemption(deps, env, info)
                }
                VaultenatorExtensionExecuteMsg::CreateRedemption { amount } => {
                    handle_create_redemption(deps, env, info, amount)
                }
                VaultenatorExtensionExecuteMsg::ClaimOwnership {} => StructuredVault
                    .handle_claim_ownership(deps, info, env, OWNER, OWNERSHIP_PROPOSAL),
                VaultenatorExtensionExecuteMsg::RegisterSudo {} => {
                    handle_register_sudo(deps, env, info)
                }
                VaultenatorExtensionExecuteMsg::Crank {} => {
                    StructuredVault.handle_crank(deps, env, info)
                }
                VaultenatorExtensionExecuteMsg::Pause {} => {
                    StructuredVault.handle_pause_contract(deps, info)
                }
                VaultenatorExtensionExecuteMsg::ProposeNewOwner {
                    new_owner,
                    duration,
                } => StructuredVault.handle_ownership_proposal(
                    deps,
                    info,
                    env,
                    new_owner,
                    duration,
                    OWNER,
                    OWNERSHIP_PROPOSAL,
                ),
                VaultenatorExtensionExecuteMsg::RejectOwner {} => StructuredVault
                    .handle_ownership_proposal_rejection(deps, info, OWNER, OWNERSHIP_PROPOSAL),
                VaultenatorExtensionExecuteMsg::SetOpen {} => {
                    StructuredVault.handle_open_contract(deps, info)
                }
                VaultenatorExtensionExecuteMsg::Withdraw { tokens_to_withdraw } => {
                    handle_withdraw(deps, env, info, tokens_to_withdraw)
                }

                VaultenatorExtensionExecuteMsg::Repay { cycle_profit } => {
                    handle_repay(deps, env, info, cycle_profit)
                }
                VaultenatorExtensionExecuteMsg::RepayQueue {
                    cycle_profit,
                    max_queue_amount,
                } => handle_repay_queue(deps, env, info, cycle_profit, max_queue_amount),
                VaultenatorExtensionExecuteMsg::Unpause {} => {
                    StructuredVault.handle_unpause_contract(deps, info)
                }
                VaultenatorExtensionExecuteMsg::UpdateConfig { new_config } => {
                    Config::handle_update_config(&mut deps, info, env, new_config)
                }
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VaultStandardInfo {} => to_json_binary(&VaultStandardInfoResponse {
            version: StructuredVault::VAULT_STANDARD_VERSION.to_string(),
            extensions: StructuredVault::VAULT_STANDARD_EXTENSIONS
                .iter()
                .map(|&s| s.into())
                .collect(),
        }),
        QueryMsg::Info {} => to_json_binary(&StructuredVault::query_info(deps, env)?),
        #[allow(deprecated)]
        QueryMsg::PreviewDeposit { .. } => {
            unimplemented!("PreviewDeposit is deprecated")
        }
        #[allow(deprecated)]
        QueryMsg::PreviewRedeem { .. } => {
            unimplemented!("PreviewRedeem is deprecated")
        }
        QueryMsg::VaultTokenExchangeRate { .. } => {
            unimplemented!("VaultTokenExchangeRate is not implemented")
        }
        QueryMsg::TotalAssets {} => {
            to_json_binary(&StructuredVault::query_total_assets(deps, env)?)
        }
        QueryMsg::TotalVaultTokenSupply {} => {
            to_json_binary(&StructuredVault::query_total_vault_token_supply(deps, env)?)
        }
        QueryMsg::ConvertToShares { amount } => to_json_binary(
            &StructuredVault::query_convert_to_shares(amount, deps, env)?,
        ),
        QueryMsg::ConvertToAssets { amount } => to_json_binary(
            &StructuredVault::query_convert_to_assets(amount, deps, env)?,
        ),
        QueryMsg::VaultExtension(msg) => match msg {
            ExtensionQueryMsg::Vaultenator(msg) => match msg {
                VaultenatorExtensionQueryMsg::Config {} => {
                    to_json_binary(&StructuredVault::query_config(deps)?)
                }
                VaultenatorExtensionQueryMsg::EstimateVaultAssets { amount } => {
                    to_json_binary(&query_estimate_vault_assets(amount, deps, env)?)
                }
                VaultenatorExtensionQueryMsg::Owner {} => {
                    to_json_binary(&StructuredVault::query_owner(deps)?)
                }
                VaultenatorExtensionQueryMsg::OwnershipProposal {} => to_json_binary(
                    &StructuredVault::query_ownership_proposal(deps, OWNERSHIP_PROPOSAL)?,
                ),
                VaultenatorExtensionQueryMsg::PendingRedemptions { limit } => {
                    to_json_binary(&query_pending_redemptions(deps, limit)?)
                }
                VaultenatorExtensionQueryMsg::State {} => {
                    to_json_binary(&query_state_wrapper(deps)?)
                }
                VaultenatorExtensionQueryMsg::WithdrawableAmount {} => {
                    to_json_binary(&query_withdrawable_amount(deps, env)?)
                }
                VaultenatorExtensionQueryMsg::UserRedemption { user } => {
                    to_json_binary(&query_user_redemption(deps, user)?)
                }
                VaultenatorExtensionQueryMsg::Version {} => to_json_binary(&query_version(deps)?),
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    StructuredVault.handle_reply(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    StructuredVault.handle_migrate(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BlockBeforeSend { from, to, amount } => {
            sudo_block_before_send(deps, env, from, to, amount)
        }
    }
}

use crate::{
    errors::ContractError,
    events::{event_repay, event_set_grants, event_set_vault, event_withdraw},
    state::CONFIG,
    utils::{create_authz_grant_messages, revoke_authz_grant_messages, tokens_to_string},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure, to_json_binary, Coin, Decimal, DepsMut, Env, MessageInfo, Response, StdError, WasmMsg,
};
use cw_utils::nonpayable;
use cw_vault_standard::VaultStandardExecuteMsg;

pub type VaultExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;

#[cw_serde]
pub enum ExtensionExecuteMsg {
    Vaultenator(VaultenatorExtensionExecuteMsg),
}

#[cw_serde]
pub enum VaultenatorExtensionExecuteMsg {
    Withdraw { tokens_to_withdraw: Vec<Coin> },
    Repay { cycle_profit: Option<Decimal> },
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

    let vault = match &config.vault {
        Some(vault) => vault,
        None => {
            return Err(ContractError::VaultNotSet {});
        }
    };

    let msg = WasmMsg::Execute {
        msg: to_json_binary(&VaultExecuteMsg::VaultExtension(
            ExtensionExecuteMsg::Vaultenator(VaultenatorExtensionExecuteMsg::Withdraw {
                tokens_to_withdraw: tokens_to_withdraw.clone(),
            }),
        ))?,
        funds: vec![],
        contract_addr: vault.to_string(),
    };

    Ok(Response::default()
        .add_event(event_withdraw(tokens_to_string(tokens_to_withdraw)))
        .add_message(msg))
}

pub fn handle_repay(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    tokens_to_repay: Vec<Coin>,
    cycle_profit: Option<Decimal>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    ensure!(
        config.controller == info.sender,
        ContractError::Unauthorized {}
    );

    let vault = match &config.vault {
        Some(vault) => vault,
        None => {
            return Err(ContractError::VaultNotSet {});
        }
    };

    let msg = WasmMsg::Execute {
        msg: to_json_binary(&VaultExecuteMsg::VaultExtension(
            ExtensionExecuteMsg::Vaultenator(VaultenatorExtensionExecuteMsg::Repay {
                cycle_profit,
            }),
        ))?,
        funds: tokens_to_repay.clone(),
        contract_addr: vault.to_string(),
    };

    Ok(Response::default()
        .add_event(event_repay(tokens_to_string(tokens_to_repay)))
        .add_message(msg))
}

pub fn handle_set_vault(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    vault: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    ensure!(config.admin == info.sender, ContractError::Unauthorized {});

    config.vault = Some(vault.clone());
    config.validate(&deps.as_ref())?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_event(event_set_vault(vault)))
}

pub fn handle_set_grants(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    grants: Vec<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    ensure!(config.admin == info.sender, ContractError::Unauthorized {});

    let mut response = Response::new();

    let revoke_msgs = revoke_authz_grant_messages(
        env.contract.address.as_str(),
        &config.controller,
        config.grants.clone(),
    );

    response = response.add_messages(revoke_msgs);

    config.grants.clone_from(&grants);
    config.validate(&deps.as_ref())?;

    let grantee = config.controller.clone();
    let grants_str: Vec<&str> = config.grants.iter().map(|s| s.as_str()).collect();

    let authz_msgs =
        create_authz_grant_messages(env.contract.address.as_str(), &grantee, &grants_str);

    CONFIG.save(deps.storage, &config)?;

    Ok(response
        .add_event(event_set_grants(grants))
        .add_messages(authz_msgs))
}

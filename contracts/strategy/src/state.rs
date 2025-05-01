use crate::utils::create_authz_allow_list_messages;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Deps, DepsMut, Env, Response, StdError, StdResult};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use interface::strategy::{OwnerProposal, PoolInfo};
use neutron_std::types::cosmos::bank::v1beta1::SendAuthorization;
use std::collections::HashSet;

pub const OWNER: Admin = Admin::new("owner");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct V010Config {
    pub admin: String,
    pub controller: String,
    pub vault: Option<String>,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub pool_info: PoolInfo,
}

#[cw_serde]
pub struct Config {
    pub admin: String,
    pub controller: String,
    pub vault: Option<String>,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub send_authorization: Option<SendAuthorization>,
    pub pool_info: PoolInfo,
}

impl Config {
    pub fn validate(&self, deps: &Deps) -> StdResult<()> {
        deps.api.addr_validate(&self.admin)?;
        deps.api.addr_validate(&self.controller)?;

        if self.vault.is_some() {
            deps.api.addr_validate(self.vault.as_ref().unwrap())?;
        }

        ensure!(
            !self.grants.is_empty(),
            StdError::generic_err("Grants must be non-empty")
        );

        ensure_no_duplicates(self.grants.clone())?;

        Ok(())
    }
}

pub fn ensure_no_duplicates(input: Vec<String>) -> StdResult<()> {
    let mut seen = HashSet::new();

    for item in &input {
        if !seen.insert(item) {
            return Err(StdError::generic_err("Duplicate grants are not allowed"));
        }
    }

    Ok(())
}

pub fn migrate_config(
    deps: DepsMut,
    env: Env,
    send_authorization: Option<SendAuthorization>,
) -> StdResult<Response> {
    let old_config: Item<V010Config> = Item::new("config");

    let cfg = old_config.load(deps.storage)?;

    let new_config = Config {
        admin: cfg.admin,
        controller: cfg.controller,
        vault: cfg.vault,
        token0: cfg.token0,
        token1: cfg.token1,
        grants: cfg.grants,
        send_authorization: send_authorization.clone(),
        pool_info: cfg.pool_info,
    };

    let mut response = Response::new();
    if let Some(send_authorization) = send_authorization {
        let authz_msgs = create_authz_allow_list_messages(
            env.contract.address.as_str(),
            &new_config.controller,
            &send_authorization,
        );
        response = response.add_message(authz_msgs);
    }

    new_config.validate(&deps.as_ref())?;
    CONFIG.save(deps.storage, &new_config)?;

    Ok(response)
}

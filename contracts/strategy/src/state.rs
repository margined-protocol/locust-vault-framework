use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Deps, StdError, StdResult};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use interface::strategy::{OwnerProposal, PoolInfo};
use std::collections::HashSet;

pub const OWNER: Admin = Admin::new("owner");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub admin: String,
    pub controller: String,
    pub vault: Option<String>,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
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

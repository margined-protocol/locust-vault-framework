use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, StdError, StdResult};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use interface::redemption::{FundInfo, OwnerProposal};
use std::collections::HashSet;

pub const OWNER: Admin = Admin::new("owner");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub admin: String,
    pub whitelisted_funds: Vec<FundInfo>,
}

impl Config {
    pub fn validate(&self, deps: &Deps) -> StdResult<()> {
        deps.api.addr_validate(&self.admin)?;

        ensure_no_duplicates(self.whitelisted_funds.clone())?;

        Ok(())
    }
}

pub fn ensure_no_duplicates(input: Vec<FundInfo>) -> StdResult<()> {
    let mut seen = HashSet::new();

    for item in &input {
        if !seen.insert(item.address.clone()) {
            return Err(StdError::generic_err(
                "Duplicate fund contracts are not allowed",
            ));
        }
    }

    Ok(())
}

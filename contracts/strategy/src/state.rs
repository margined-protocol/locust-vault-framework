use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Deps, StdError, StdResult};
use cw_storage_plus::Item;
use interface::strategy::PoolInfo;

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub controller: String,
    pub vault: String,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub pool_info: PoolInfo,
}

impl Config {
    pub fn validate(&self, deps: &Deps) -> StdResult<()> {
        deps.api.addr_validate(&self.controller)?;
        deps.api.addr_validate(&self.vault)?;

        ensure!(
            !self.grants.is_empty(),
            StdError::generic_err("Grants must be non-empty")
        );

        Ok(())
    }
}

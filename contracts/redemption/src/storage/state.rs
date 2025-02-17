use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, StdError, StdResult};
use cw_controllers::Admin;
use cw_storage_plus::Item;
use interface::redemption::{FundInfo, OwnerProposal};
use std::collections::HashSet;

pub const OWNER: Admin = Admin::new("owner");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONFIG: Item<Config> = Item::new("config");

pub const MAX_METADATA_LENGTH: usize = 255;
pub const MAX_WHITELISTED_FUNDS: usize = 100;

#[cw_serde]
pub struct Config {
    pub admin: String,
    // ASSUMPTION: no more than 100 whitelisted funds
    pub whitelisted_funds: Vec<FundInfo>,
}

impl Config {
    pub fn validate(&self, deps: &Deps) -> StdResult<()> {
        // Validate admin is a valid address
        deps.api.addr_validate(&self.admin)?;

        // Validate number of whitelisted funds
        if self.whitelisted_funds.len() > MAX_WHITELISTED_FUNDS {
            return Err(StdError::generic_err(format!(
                "Number of whitelisted funds exceeds maximum of {}",
                MAX_WHITELISTED_FUNDS
            )));
        }

        // Validate whitelisted funds
        self.validate_whitelisted_funds(deps)?;

        Ok(())
    }

    fn validate_whitelisted_funds(&self, deps: &Deps) -> StdResult<()> {
        // Check for duplicates
        ensure_no_duplicates(&self.whitelisted_funds)?;

        // Validate each fund
        for fund in &self.whitelisted_funds {
            // Validate address

            deps.api.addr_validate(&fund.address)?;

            // Check metadata length
            if fund.metadata.len() > MAX_METADATA_LENGTH {
                return Err(StdError::generic_err(format!(
                    "Metadata length exceeds maximum of {} characters",
                    MAX_METADATA_LENGTH
                )));
            }
        }

        Ok(())
    }
}

pub fn ensure_no_duplicates(input: &[FundInfo]) -> StdResult<()> {
    let mut seen = HashSet::new();

    for item in input {
        if !seen.insert(item.address.clone()) {
            return Err(StdError::generic_err(
                "Duplicate fund contracts are not allowed",
            ));
        }
    }

    Ok(())
}

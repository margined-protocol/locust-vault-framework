use cosmwasm_std::{Deps, Order, StdError, StdResult};
use cw2::get_contract_version;
use cw_storage_plus::Bound;
use interface::redemption::{ConfigResponse, PendingRedemption};

use crate::{
    errors::ContractError,
    storage::{
        redemptions::{filter_user_redemptions, redemptions},
        state::{CONFIG, OWNER},
    },
};

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 30;

pub fn query_config(deps: &Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let version = get_contract_version(deps.storage)?;

    Ok(ConfigResponse {
        admin: config.admin,
        whitelisted_funds: config.whitelisted_funds,
        name: version.contract,
        version: version.version,
    })
}

pub fn query_owner(deps: Deps) -> Result<String, ContractError> {
    if let Some(owner) = OWNER.get(deps)? {
        Ok(owner.to_string())
    } else {
        Err(ContractError::Std(StdError::generic_err("Owner not set")))
    }
}

pub fn query_all_redemptions(
    deps: Deps,
    start_after: Option<(String, u64)>,
    limit: Option<u32>,
) -> StdResult<Vec<PendingRedemption>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    // let start = start_after.map(|s| (s.0, s.1));

    let start = start_after
        .as_ref()
        .map(|s| Bound::exclusive((s.0.as_str(), s.1)));

    let redemptions = redemptions()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, redemption)| redemption))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(redemptions)
}

pub fn query_redemptions(
    deps: Deps,
    user: String,
    limit: Option<u32>,
) -> StdResult<Vec<PendingRedemption>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let redemptions = filter_user_redemptions(deps.storage, user)
        .take(limit)
        .map(|item| item.map(|(_, redemption)| redemption))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(redemptions)
}

use crate::{
    helpers::calculate_amount_withdrawable,
    queries::external::{get_balance, get_total_supply},
    storage::{
        config::Config,
        queue::{get_all_redemptions, get_all_user_redemptions, Redemption},
        state::State,
    },
};

use cosmwasm_std::{Coin, Decimal, Deps, Env, StdError, StdResult, Uint128};
use cw2::get_contract_version;
use cw_storage_plus::Bound;
use interface::fund::{StateResponse, VersionResponse};
use vaultenator::{config::Configure, state::ManageState};

pub const DEFAULT_LIMIT: u32 = 150u32;

pub fn query_estimate_vault_assets(amount: Uint128, deps: Deps, env: Env) -> StdResult<Vec<Coin>> {
    let config =
        Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;
    let state = State::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

    let total_supply = get_total_supply(&deps, &config.strategy_denom)?;
    let share = Decimal::from_ratio(amount, total_supply);

    let denoms = config.get_denoms();

    let mut assets = Vec::new();
    for token in denoms {
        let token_balance = get_balance(&deps, env.contract.address.as_str(), &token)?;

        let total_withdrawn = state
            .total_withdrawn_tokens
            .get(&token)
            .cloned()
            .unwrap_or(Uint128::zero());

        let current_assets = token_balance.checked_add(total_withdrawn)?;

        let amount_to_redeem = Coin::new(current_assets.mul_floor(share), &token);

        assets.push(amount_to_redeem);
    }

    Ok(assets)
}

pub fn query_pending_redemptions(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Redemption>> {
    let query_limit = limit.unwrap_or(DEFAULT_LIMIT) as usize;

    let start_bound = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    get_all_redemptions(deps.storage, start_bound, query_limit)
}

pub fn query_state_wrapper(deps: Deps) -> StdResult<StateResponse> {
    let state = State::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

    let total_withdrawn_tokens = state
        .total_withdrawn_tokens
        .iter()
        .map(|(k, v)| Coin {
            denom: k.clone(),
            amount: *v,
        })
        .collect();

    Ok(StateResponse {
        is_open: state.is_open,
        is_paused: state.is_paused,
        last_pause: state.last_pause,
        last_claim: state.last_claim,
        total_staked_tokens: state.total_staked_tokens,
        pending_management_fees: state.pending_management_fees,
        total_withdrawn_tokens,
    })
}

pub fn query_user_redemption(deps: Deps, user: String) -> StdResult<Vec<Redemption>> {
    let user = deps.api.addr_validate(&user)?;

    get_all_user_redemptions(deps.storage, user)
}

pub fn query_version(deps: Deps) -> StdResult<VersionResponse> {
    let res = get_contract_version(deps.storage)?;

    Ok(VersionResponse {
        name: res.contract,
        version: res.version,
    })
}

pub fn query_withdrawable_amount(deps: Deps, env: Env) -> StdResult<Vec<Coin>> {
    let config =
        Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;
    let state = State::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

    let tokens = if let Some(token1) = config.token1.clone() {
        vec![config.token0.clone(), token1]
    } else {
        vec![config.token0.clone()]
    };

    let mut assets = Vec::new();
    for token in tokens {
        let amount_withdrawable = calculate_amount_withdrawable(
            &deps,
            &config,
            &state,
            env.contract.address.as_str(),
            &token,
        )?;

        assets.push(Coin::new(amount_withdrawable, &token));
    }

    Ok(assets)
}

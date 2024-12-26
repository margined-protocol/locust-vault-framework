use crate::{
    contract::StructuredVault,
    helpers::{calculate_amount_withdrawable, calculate_assets_value},
    math::get_amount_to_mint,
    queries::{get_balance, get_total_supply},
    storage::{config::Config, state::State},
};

use cosmwasm_std::{Coin, Decimal, Deps, Env, StdError, StdResult, Uint128};
use cw2::get_contract_version;
use cw_vault_standard::msg::VaultInfoResponse;
use interface::fund::{StateResponse, VersionResponse};
use vaultenator::{config::Configure, query::Query, state::ManageState};

impl Query<Config, State> for StructuredVault {
    fn query_info(deps: Deps, _env: Env) -> StdResult<VaultInfoResponse> {
        let config =
            Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

        Ok(VaultInfoResponse {
            base_token: config.token0,
            vault_token: config.strategy_denom,
        })
    }

    fn query_preview_deposit(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128> {
        let config =
            Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

        let current_assets = get_balance(&deps, env.contract.address.as_str(), &config.token0)?;

        let previous_assets = current_assets.checked_sub(amount)?;

        get_amount_to_mint(
            deps,
            &current_assets,
            &previous_assets,
            &config.strategy_denom,
        )
    }

    fn query_preview_redeem(_amount: Uint128, _deps: Deps, _env: Env) -> StdResult<Uint128> {
        unimplemented!()
    }

    fn query_total_assets(deps: Deps, env: Env) -> StdResult<Uint128> {
        let config =
            Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;
        let state =
            State::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

        let token = get_balance(&deps, env.contract.address.as_str(), &config.token0)?;

        token
            .checked_add(state.get_total_withdrawn_tokens(&config.token0))
            .map_err(|e| StdError::generic_err(e.to_string()))
    }

    fn query_total_vault_token_supply(deps: Deps, _env: Env) -> StdResult<Uint128> {
        let config =
            Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

        get_total_supply(&deps, &config.strategy_denom)
    }

    fn query_convert_to_shares(_amount: Uint128, _deps: Deps, _env: Env) -> StdResult<Uint128> {
        unimplemented!()
    }

    // NOTE: this is still todo, as it needs to account for other deposited assets
    fn query_convert_to_assets(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128> {
        let config =
            Config::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;
        let state =
            State::get_from_storage(deps).map_err(|e| StdError::generic_err(e.to_string()))?;

        let total_supply = get_total_supply(&deps, &config.strategy_denom)?;

        if amount > total_supply {
            return Err(StdError::generic_err("Amount exceeds total supply"));
        }

        let current_assets =
            calculate_assets_value(&deps, &config, &state, env.contract.address.as_ref())?;

        let share = Decimal::from_ratio(amount, total_supply);

        Ok(current_assets.mul_floor(share))
    }
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

use crate::queries::get_total_supply;

use cosmwasm_std::{Decimal, Deps, StdResult, Uint128};

pub const YEAR_IN_SECONDS: u64 = 365 * 24 * 60 * 60;

pub fn get_amount_to_mint(
    deps: Deps,
    current_assets: &Uint128,
    previous_assets: &Uint128,
    strategy_denom: &str,
) -> StdResult<Uint128> {
    let total_supply = get_total_supply(&deps, strategy_denom)?;
    let amount_to_mint = calculate_amount_to_mint(current_assets, previous_assets, total_supply);

    Ok(amount_to_mint)
}

pub fn calculate_amount_to_mint(
    current_assets: &Uint128,
    previous_assets: &Uint128,
    total_supply: Uint128,
) -> Uint128 {
    let delta_liquidity = current_assets.saturating_sub(*previous_assets);
    let normalized_delta = Decimal::from_ratio(delta_liquidity, *previous_assets);

    total_supply.mul_floor(normalized_delta)
}

pub fn calculate_management_fee(
    amount: Uint128,
    management_fee: Decimal,
    time_elapsed_seconds: u64,
    year_in_seconds: u64,
) -> Uint128 {
    let elapsed_time_multiplier =
        Decimal::from_ratio(time_elapsed_seconds as u128, year_in_seconds as u128);

    let fee_multiplier = management_fee * elapsed_time_multiplier;

    amount.mul_floor(fee_multiplier)
}

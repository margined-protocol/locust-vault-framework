use crate::queries::external::get_total_supply;

use core::str::FromStr;
use cosmwasm_std::{Decimal, Deps, Int128, SignedDecimal, StdResult, Uint128};

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

// Helper function to calculate decimal rate with sign handling
pub fn apply_pnl(amount: Uint128, pnl: SignedDecimal) -> Uint128 {
    let abs_rate =
        Decimal::from_atomics(pnl.atomics().abs().i128() as u128, pnl.decimal_places()).unwrap();

    let pnl_amount = amount.mul_floor(abs_rate);

    // Counterintuitive logic:
    // If PnL is negative then the amount is increased, as we want to reduce the total outstanding amount
    // This is because the pnl is a loss, so we need to add it back to the amount to get the correct amount
    // If PnL is positive then the amount is decreased, as we want to reduce the total outstanding amount
    // This is because the pnl is a profit, so we need to subtract it from the amount to get the correct amount
    let result = if pnl.is_negative() {
        amount.saturating_add(pnl_amount)
    } else {
        amount.saturating_sub(pnl_amount)
    };

    result
}

// Helper function to convert Decimal to SignedDecimal
pub fn decimal_to_signed(decimal: Decimal) -> SignedDecimal {
    SignedDecimal::from_str(&decimal.to_string()).unwrap_or(SignedDecimal::zero())
}

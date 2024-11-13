use crate::math::{calculate_management_fee, YEAR_IN_SECONDS};

use cosmwasm_std::{Decimal, Uint128};

#[test]
fn test_calculate_management_fee_for_one_day() {
    let amount = Uint128::from(1_000_000_000u128);
    let time_elapsed_seconds = 86400; // 1 day
    let management_fee = Decimal::percent(2);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    // 1,000,000,000 * 2% * 1 day / 365 days = ~54794.52 (rounds to 54794)
    assert_eq!(fee, Uint128::from(54_794u128));
}

#[test]
fn test_calculate_management_fee_for_one_year() {
    let amount = Uint128::from(1_000_000_000u128);
    let time_elapsed_seconds = 31_536_000; // 1 year
    let management_fee = Decimal::percent(2);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    // 1,000,000,000 * 2% * 1 year / 365 days = 20,000,000
    assert_eq!(fee, Uint128::from(20_000_000u128));
}

#[test]
fn test_calculate_management_fee_for_half_year() {
    let amount = Uint128::from(1_000_000_000u128);
    let time_elapsed_seconds = 15_768_000; // Half a year
    let management_fee = Decimal::percent(2);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    // 1,000,000,000 * 2% * 0.5 year / 365 days = 10,000,000
    assert_eq!(fee, Uint128::from(10_000_000u128));
}

#[test]
fn test_calculate_management_fee_for_zero_time() {
    let amount = Uint128::from(1_000_000_000u128);
    let time_elapsed_seconds = 0; // No time elapsed
    let management_fee = Decimal::percent(2);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );
    assert_eq!(fee, Uint128::zero());
}

#[test]
fn test_calculate_management_fee_for_one_hour() {
    let amount = Uint128::from(1_000_000u128);
    let time_elapsed_seconds = 3_600u64; // 1 hour
    let management_fee = Decimal::percent(1);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    assert_eq!(fee, Uint128::from(1u128));
}

#[test]
fn test_calculate_management_minimum_time_elapsed_without_losing_precision() {
    let amount = Uint128::from(1_000_000u128);
    let time_elapsed_seconds = 3_154u64; // 1 second
    let management_fee = Decimal::percent(1);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    assert_eq!(fee, Uint128::from(1u128));

    let time_elapsed_seconds = 3_153u64; // 1 second
    let management_fee = Decimal::percent(1);

    let fee = calculate_management_fee(
        amount,
        management_fee,
        time_elapsed_seconds,
        YEAR_IN_SECONDS,
    );

    assert_eq!(fee, Uint128::zero());
}

use crate::math::{apply_pnl, calculate_management_fee, decimal_to_signed, YEAR_IN_SECONDS};

use cosmwasm_std::{Decimal, SignedDecimal, Uint128};

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

#[test]
fn test_apply_pnl_positive_profit() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::percent(10); // 10% profit

    let (amount, profit) = apply_pnl(amount, pnl);
    assert_eq!(amount, Uint128::from(1000u128)); // 1000 - (10% of 1000)
    assert_eq!(profit, Uint128::from(100u128));
}

#[test]
fn test_apply_pnl_negative_loss() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::percent(-10); // -10% loss
    let (amount, profit) = apply_pnl(amount, pnl);
    assert_eq!(amount, Uint128::from(1100u128)); // 1000 - (10% of 1000)
    assert_eq!(profit, Uint128::zero());
}

#[test]
fn test_apply_pnl_zero() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::zero();
    let (amount, profit) = apply_pnl(amount, pnl);

    assert_eq!(amount, Uint128::from(1000u128)); // Amount should remain unchanged
    assert_eq!(profit, Uint128::zero());
}

#[test]
fn test_apply_pnl_large_amount() {
    let amount = Uint128::from(1_000_000_000u128);
    let pnl = SignedDecimal::percent(50); // 50% profit
    let (amount, profit) = apply_pnl(amount, pnl);

    assert_eq!(amount, Uint128::from(1_000_000_000u128)); // 1B + (50% of 1B)
    assert_eq!(profit, Uint128::from(500_000_000u128)); // 1B - (50% of 1B)
}

#[test]
fn test_apply_pnl_small_rate() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::percent(1); // 1% profit
    let (amount, profit) = apply_pnl(amount, pnl);

    assert_eq!(amount, Uint128::from(1000u128)); // Amount should remain unchanged
    assert_eq!(profit, Uint128::from(10u128)); //  (1% of 1000)
}

#[test]
fn test_apply_pnl_100_percent() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::percent(100); // 100% profit
    let (amount, profit) = apply_pnl(amount, pnl);

    assert_eq!(amount, Uint128::from(1000u128)); // Amount should double with 100% profit
    assert_eq!(profit, Uint128::from(1000u128)); // Should be zero after 100% profit
}

#[test]
fn test_apply_pnl_negative_100_percent() {
    let amount = Uint128::from(1000u128);
    let pnl = SignedDecimal::percent(-100); // -100% loss
    let (amount, profit) = apply_pnl(amount, pnl);

    assert_eq!(amount, Uint128::from(2000u128)); // Amount should double with 100% profit
    assert_eq!(profit, Uint128::zero()); // Should double with 100% loss
}

#[test]
fn test_decimal_to_signed_positive() {
    let decimal = Decimal::percent(10); // 10%
    let signed = decimal_to_signed(decimal);
    assert_eq!(signed, SignedDecimal::percent(10));
}

#[test]
fn test_decimal_to_signed_zero() {
    let decimal = Decimal::zero();
    let signed = decimal_to_signed(decimal);
    assert_eq!(signed, SignedDecimal::zero());
}

#[test]
fn test_decimal_to_signed_small() {
    let decimal = Decimal::percent(1); // 1%
    let signed = decimal_to_signed(decimal);
    assert_eq!(signed, SignedDecimal::percent(1));
}

#[test]
fn test_decimal_to_signed_large() {
    let decimal = Decimal::percent(100); // 100%
    let signed = decimal_to_signed(decimal);
    assert_eq!(signed, SignedDecimal::percent(100));
}

#[test]
fn test_decimal_to_signed_precision() {
    let decimal = Decimal::from_ratio(1u128, 3u128); // 0.333...
    let signed = decimal_to_signed(decimal);
    // Compare string representations to handle precision differences
    assert_eq!(signed.to_string(), "0.333333333333333333");
}

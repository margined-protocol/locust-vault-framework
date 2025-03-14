use crate::helpers::{calculate_amount_to_mint, get_management_fees};

use cosmwasm_std::{coin, Decimal, Uint128};

#[test]
fn test_fees_with_sufficient_balance() {
    let latest_timestamp = 31_536_000; // 1 year
    let last_claim = 0;
    let total_assets = vec![coin(1000, "token1")];
    let balances = vec![coin(1000, "token1")];
    let pending_fees = vec![];
    let management_fee = Decimal::percent(2); // 2% management fee

    let (fees, pending_fees) = get_management_fees(
        latest_timestamp,
        last_claim,
        total_assets,
        balances,
        pending_fees,
        management_fee,
    )
    .unwrap();

    assert_eq!(fees, vec![coin(20, "token1")]); // 2% of 1000 is 20
    assert!(pending_fees.is_empty());
}

#[test]
fn test_fees_exceeding_balance() {
    let latest_timestamp = 31_536_000; // 1 year
    let last_claim = 0;
    let total_assets = vec![coin(1000, "token1")];
    let balances = vec![coin(10, "token1")]; // Small balance
    let pending_fees = vec![];
    let management_fee = Decimal::percent(2); // 2% management fee

    let (fees, pending_fees) = get_management_fees(
        latest_timestamp,
        last_claim,
        total_assets,
        balances,
        pending_fees,
        management_fee,
    )
    .unwrap();

    assert_eq!(fees, vec![coin(10, "token1")]); // Only balance amount is charged
    assert_eq!(pending_fees, vec![coin(10, "token1")]); // Remaining 10 is saved as pending
}

#[test]
fn test_with_pending_fees() {
    let latest_timestamp = 15_768_000; // Half a year
    let last_claim = 0;
    let total_assets = vec![coin(1000, "token1")];
    let balances = vec![coin(1000, "token1")];
    let pending_fees = vec![coin(5, "token1")]; // Previous pending fees
    let management_fee = Decimal::percent(2); // 2% management fee

    let (fees, pending_fees) = get_management_fees(
        latest_timestamp,
        last_claim,
        total_assets,
        balances,
        pending_fees,
        management_fee,
    )
    .unwrap();

    // Expected fee is 10 for half year + 5 from pending fees = 15
    assert_eq!(fees, vec![coin(15, "token1")]);
    assert!(pending_fees.is_empty());
}

#[test]
fn test_partial_balance_with_pending_fees() {
    let latest_timestamp = 15_768_000; // Half a year
    let last_claim = 0;
    let total_assets = vec![coin(1000, "token1")];
    let balances = vec![coin(10, "token1")];
    let pending_fees = vec![coin(5, "token1")]; // Previous pending fees
    let management_fee = Decimal::percent(2); // 2% management fee

    let (fees, pending_fees) = get_management_fees(
        latest_timestamp,
        last_claim,
        total_assets,
        balances,
        pending_fees,
        management_fee,
    )
    .unwrap();

    // Expected fee is 10 from balance, remainder is 5 from pending + 10 left = 15 in pending
    assert_eq!(fees, vec![coin(10, "token1")]);
    assert_eq!(pending_fees, vec![coin(5, "token1")]);
}

#[test]
fn test_no_fee_due() {
    let latest_timestamp = 0;
    let last_claim = 0;
    let total_assets = vec![coin(1000, "token1")];
    let balances = vec![coin(1000, "token1")];
    let pending_fees = vec![];
    let management_fee = Decimal::percent(2); // 2% management fee

    let (fees, pending_fees) = get_management_fees(
        latest_timestamp,
        last_claim,
        total_assets,
        balances,
        pending_fees,
        management_fee,
    )
    .unwrap();

    assert!(fees.is_empty());
    assert!(pending_fees.is_empty());
}

#[test]
fn test_calculate_amount_to_mint() {
    // Test case 1: Basic case - 10% increase in assets
    let current_assets = Uint128::new(110);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(1000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(100)); // 10% of 1000 = 100

    // Test case 2: No change in assets
    let current_assets = Uint128::new(100);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(1000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(0)); // 0% change = 0 minted

    // Test case 3: Decrease in assets (should return 0 due to saturating_sub)
    let current_assets = Uint128::new(90);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(1000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(0)); // Decrease should result in 0

    // Test case 4: Large increase in assets
    let current_assets = Uint128::new(200);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(1000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(1000)); // 100% increase = 1000

    // Test case 5: Small increase with rounding
    let current_assets = Uint128::new(103);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(1000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(30)); // 3% of 1000 = 30

    // Test case 6: Panic - handled in separate test

    // Test case 7: Zero total supply
    let current_assets = Uint128::new(110);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(0);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(0)); // 0 supply means 0 minted

    // Test case 8: Very large numbers
    let current_assets = Uint128::new(1_000_000_000);
    let previous_assets = Uint128::new(500_000_000);
    let total_supply = Uint128::new(1_000_000_000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(1_000_000_000)); // 100% increase

    // Test case 9: Fractional result with rounding down
    let current_assets = Uint128::new(101);
    let previous_assets = Uint128::new(100);
    let total_supply = Uint128::new(999);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    // 1% of 999 = 9.99, should round down to 9
    assert_eq!(result, Uint128::new(9));

    // Test case 10: Worked example - USDC
    let current_assets = Uint128::new(2000000);
    let previous_assets = Uint128::new(1000000);
    let total_supply = Uint128::new(1000000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(1000000));

    // Test case 11: Worked example - USDC, incorrectly initiated
    let current_assets = Uint128::new(50010000);
    let previous_assets = Uint128::new(10000);
    let total_supply = Uint128::new(1000000);

    let result = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
    assert_eq!(result, Uint128::new(5000000000));
}

#[test]
#[should_panic(expected = "Denominator must not be zero")]
fn test_calculate_amount_to_mint_division_by_zero() {
    // Test division by zero case
    let current_assets = Uint128::new(100);
    let previous_assets = Uint128::new(0);
    let total_supply = Uint128::new(1000);

    // This should panic with "Attempt to divide by zero"
    let _ = calculate_amount_to_mint(&current_assets, &previous_assets, total_supply);
}

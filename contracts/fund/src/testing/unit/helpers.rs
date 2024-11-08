use crate::helpers::get_management_fees;

use cosmwasm_std::{coin, Decimal};

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

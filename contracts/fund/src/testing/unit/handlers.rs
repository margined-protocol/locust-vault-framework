use crate::handlers::helpers::calculate_total_assets_redeemable;

use cosmwasm_std::{Coin, Uint128};

#[test]
fn test_calculate_total_assets_redeemable_success() {
    let mut remaining_balance = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(200),
        },
    ];

    let assets_to_redeem = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(50),
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(150),
        },
    ];

    // Process assets to redeem
    let result = calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance);

    // Assert success
    assert!(result.is_ok());

    // Assert remaining balance is updated correctly
    assert_eq!(
        remaining_balance,
        vec![
            Coin {
                denom: "atom".to_string(),
                amount: Uint128::new(50), // 100 - 50
            },
            Coin {
                denom: "osmo".to_string(),
                amount: Uint128::new(50), // 200 - 150
            }
        ]
    );
}

#[test]
fn test_calculate_total_assets_redeemable_insufficient_balance() {
    let mut remaining_balance = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(200),
        },
    ];

    let assets_to_redeem = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(150), // Exceeds available balance
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(150),
        },
    ];

    // Process assets to redeem
    let result = calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance);

    // Assert failure
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert_eq!(
        error,
        "Generic error: Insufficient balance for denom 'atom'. Required: 150, Available: 100"
    );

    // Assert remaining balance is unchanged
    assert_eq!(
        remaining_balance,
        vec![
            Coin {
                denom: "atom".to_string(),
                amount: Uint128::new(100),
            },
            Coin {
                denom: "osmo".to_string(),
                amount: Uint128::new(200),
            }
        ]
    );
}

#[test]
fn test_calculate_total_assets_redeemable_missing_denomination() {
    let mut remaining_balance = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(200),
        },
    ];

    let assets_to_redeem = vec![Coin {
        denom: "juno".to_string(), // Not found in remaining_balance
        amount: Uint128::new(50),
    }];

    // Process assets to redeem
    let result = calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance);

    // Assert failure
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert_eq!(
        error,
        "Generic error: Denomination 'juno' not found in remaining balance."
    );

    // Assert remaining balance is unchanged
    assert_eq!(
        remaining_balance,
        vec![
            Coin {
                denom: "atom".to_string(),
                amount: Uint128::new(100),
            },
            Coin {
                denom: "osmo".to_string(),
                amount: Uint128::new(200),
            }
        ]
    );
}

#[test]
fn test_calculate_total_assets_redeemable_no_assets_to_redeem() {
    let mut remaining_balance = vec![
        Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(100),
        },
        Coin {
            denom: "osmo".to_string(),
            amount: Uint128::new(200),
        },
    ];

    let assets_to_redeem = vec![]; // Nothing to redeem

    // Process assets to redeem
    let result = calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance);

    // Assert success
    assert!(result.is_ok());

    // Assert remaining balance is unchanged
    assert_eq!(
        remaining_balance,
        vec![
            Coin {
                denom: "atom".to_string(),
                amount: Uint128::new(100),
            },
            Coin {
                denom: "osmo".to_string(),
                amount: Uint128::new(200),
            }
        ]
    );
}

#[test]
fn test_calculate_total_assets_redeemable_exact_balance() {
    let mut remaining_balance = vec![Coin {
        denom: "atom".to_string(),
        amount: Uint128::new(100),
    }];

    let assets_to_redeem = vec![Coin {
        denom: "atom".to_string(),
        amount: Uint128::new(100), // Matches exactly
    }];

    // Process assets to redeem
    let result = calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance);

    // Assert success
    assert!(result.is_ok());

    // Assert remaining balance is updated correctly
    assert_eq!(
        remaining_balance,
        vec![Coin {
            denom: "atom".to_string(),
            amount: Uint128::new(0), // 100 - 100
        }]
    );
}

use crate::errors::ContractError;

use cosmwasm_std::coins;
use interface::redemption::PendingRedemption;
use neutron_test_tube::{Account, Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::assert_err,
};

#[test]
fn test_basic_send() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0u64,
        source: env.fund.address(),
    };

    env.send_redemption(
        &wasm,
        &redemption_addr,
        redemption.clone(),
        &redemption.funds,
        &env.fund,
    )
    .unwrap();

    // Set the timestamp to the current block time
    let timestamp = env.app.get_block_time_seconds();
    redemption.timestamp = timestamp as u64;

    let redemptions = env
        .query_redemptions(&wasm, &redemption_addr, env.signer.address(), Some(10))
        .unwrap();
    assert_eq!(redemptions.len(), 1);
    assert_eq!(redemptions[0], redemption);
}

#[test]
fn test_multiple_users_redemptions() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    // Create redemptions for multiple users
    for (i, trader) in env.traders.iter().take(3).enumerate() {
        let mut redemption = PendingRedemption {
            user: trader.address(),
            funds: coins(100 * (i + 1) as u128, BASE_DENOM),
            timestamp: 0,
            source: env.fund.address(),
        };

        env.send_redemption(
            &wasm,
            &redemption_addr,
            redemption.clone(),
            &redemption.funds,
            &env.fund,
        )
        .unwrap();

        // Set the timestamp to the current block time
        let timestamp = env.app.get_block_time_seconds();
        redemption.timestamp = timestamp as u64;

        // Verify individual redemption
        let user_redemptions = env
            .query_redemptions(&wasm, &redemption_addr, trader.address(), Some(10))
            .unwrap();
        assert_eq!(user_redemptions.len(), 1);
        assert_eq!(user_redemptions[0], redemption);
    }

    // Verify all redemptions
    let all_redemptions = env
        .query_all_redemptions(&wasm, &redemption_addr, None, Some(10))
        .unwrap();
    assert_eq!(all_redemptions.len(), 3);
}

#[test]
fn test_pagination() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    // Create 5 redemptions
    for i in 0..5 {
        let redemption = PendingRedemption {
            user: env.signer.address(),
            funds: coins(100 * (i + 1) as u128, BASE_DENOM),
            timestamp: i as u64,
            source: env.fund.address(),
        };

        env.send_redemption(
            &wasm,
            &redemption_addr,
            redemption,
            &coins(100 * (i + 1) as u128, BASE_DENOM),
            &env.fund,
        )
        .unwrap();
    }

    // Test pagination with no limits
    let page0 = env
        .query_all_redemptions(&wasm, &redemption_addr, None, None)
        .unwrap();
    assert_eq!(page0.len(), 5);

    // Test pagination with different limits
    let page1 = env
        .query_all_redemptions(&wasm, &redemption_addr, None, Some(2))
        .unwrap();
    assert_eq!(page1.len(), 2);

    let page2 = env
        .query_all_redemptions(
            &wasm,
            &redemption_addr,
            Some((page1[1].user.clone(), page1[1].timestamp)),
            Some(2),
        )
        .unwrap();
    assert_eq!(page2.len(), 2);
    assert_ne!(page1[0], page2[0]);
}

#[test]
fn test_error_unauthorized_fund() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0,
        source: env.traders[0].address(), // Unauthorized fund
    };

    let err = env
        .send_redemption(
            &wasm,
            &redemption_addr,
            redemption,
            &coins(100, BASE_DENOM),
            &env.traders[0],
        )
        .unwrap_err();

    assert_err(err, ContractError::UnauthorizedFund {});
}

#[test]
fn test_error_invalid_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0,
        source: env.fund.address(),
    };

    // Send with incorrect funds
    let err = env
        .send_redemption(
            &wasm,
            &redemption_addr,
            redemption,
            &coins(50, BASE_DENOM), // Different amount than specified
            &env.fund,
        )
        .unwrap_err();

    assert_err(err, ContractError::InsufficientFunds {});
}

#[test]
fn test_error_claim_nonexistent() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let err = env
        .claim_redemption(&wasm, &redemption_addr, Some(1), &env.signer)
        .unwrap_err();

    assert_err(err, ContractError::NoRedemptionsFound {});
}

#[test]
fn test_basic_send_with_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    // Get initial balances
    let initial_fund_base = env.get_balance(&env.fund.address(), BASE_DENOM);
    let initial_fund_quote = env.get_balance(&env.fund.address(), QUOTE_DENOM);
    let initial_contract_base = env.get_balance(&redemption_addr, BASE_DENOM);
    let initial_contract_quote = env.get_balance(&redemption_addr, QUOTE_DENOM);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: vec![coins(100, BASE_DENOM), coins(200, QUOTE_DENOM)]
            .into_iter()
            .flatten()
            .collect(),
        timestamp: 0,
        source: env.fund.address(),
    };

    // Send redemption with multiple tokens
    env.send_redemption(
        &wasm,
        &redemption_addr,
        redemption.clone(),
        &redemption.funds,
        &env.fund,
    )
    .unwrap();

    // Verify fund's balance was reduced by correct amount
    let final_fund_base = env.get_balance(&env.fund.address(), BASE_DENOM);
    let final_fund_quote = env.get_balance(&env.fund.address(), QUOTE_DENOM);
    assert_eq!(initial_fund_base.u128() - final_fund_base.u128(), 100);
    assert_eq!(initial_fund_quote.u128() - final_fund_quote.u128(), 200);

    // Verify contract's balance increased by correct amount
    let final_contract_base = env.get_balance(&redemption_addr, BASE_DENOM);
    let final_contract_quote = env.get_balance(&redemption_addr, QUOTE_DENOM);
    assert_eq!(
        final_contract_base.u128() - initial_contract_base.u128(),
        100
    );
    assert_eq!(
        final_contract_quote.u128() - initial_contract_quote.u128(),
        200
    );
}

#[test]
fn test_error_insufficient_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0,
        source: env.fund.address(),
    };

    // Attempt to send redemption with insufficient funds
    let err = env
        .send_redemption(
            &wasm,
            &redemption_addr,
            redemption,
            &coins(50, BASE_DENOM),
            &env.fund,
        )
        .unwrap_err();

    assert_err(err, ContractError::InsufficientFunds {});
}

#[test]
fn test_error_no_funds_sent() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0,
        source: env.fund.address(),
    };

    // Attempt to send redemption without any funds
    let err = env
        .send_redemption(
            &wasm,
            &redemption_addr,
            redemption,
            &[], // Empty funds
            &env.fund,
        )
        .unwrap_err();

    assert_err(err, ContractError::InsufficientFunds {});
}

#[test]
fn test_claim_with_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let redemption = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 0,
        source: env.fund.address(),
    };

    // Send redemption
    env.send_redemption(
        &wasm,
        &redemption_addr,
        redemption.clone(),
        &redemption.funds,
        &env.fund,
    )
    .unwrap();

    // Get initial balances
    let initial_user_balance = env.get_balance(&env.signer.address(), BASE_DENOM);
    let initial_contract_balance = env.get_balance(&redemption_addr, BASE_DENOM);

    // Claim redemption
    env.claim_redemption(&wasm, &redemption_addr, Some(1), &env.signer)
        .unwrap();

    // Verify funds were transferred to user
    let final_user_balance = env.get_balance(&env.signer.address(), BASE_DENOM);
    assert_eq!(final_user_balance.u128(), initial_user_balance.u128() + 100);

    // Verify contract balance was reduced
    let final_contract_balance = env.get_balance(&redemption_addr, BASE_DENOM);
    assert_eq!(
        final_contract_balance.u128(),
        initial_contract_balance.u128() - 100
    );
}

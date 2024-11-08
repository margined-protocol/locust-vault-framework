use cosmwasm_std::{assert_approx_eq, coin, coins, Uint128};
use interface::fund::StateResponse;
use osmosis_test_tube::{Account, Module, Wasm};
use testing::setup::{TestEnv, BASE_DENOM, QUOTE_DENOM};

#[test]
fn test_redeem() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.total_staked_tokens, deposit.amount);

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(!trader_strategy_before.is_zero());

    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());
    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_approx_eq!(contract_base_before, contract_base_after, "1");
}

#[test]
fn test_redeem_twice() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.total_staked_tokens, deposit.amount);

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(!trader_strategy_before.is_zero());

    let redeem = coin(
        trader_strategy_before.u128() / 2u128,
        config.strategy_denom.clone(),
    );
    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let redeem = coin(
        trader_strategy_before.u128() / 2u128,
        config.strategy_denom.clone(),
    );
    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_approx_eq!(contract_base_before, contract_base_after, "1");
}

#[test]
fn test_redeem_multiple_users() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let deposit_1 = coin(200_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit_1.clone()], &env.traders[1])
        .unwrap();

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(!trader_strategy_before.is_zero());

    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());
    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let trader_1_strategy_after =
        env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(!trader_1_strategy_after.is_zero());

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.total_staked_tokens, deposit_1.amount);

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_approx_eq!(
        contract_base_before.checked_add(deposit_1.amount).unwrap(),
        contract_base_after,
        "1"
    );
}

#[test]
fn test_redeem_both_users() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let deposit_1 = coin(200_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit_1.clone()], &env.traders[1])
        .unwrap();

    let redeem = coin(
        env.get_balance(&env.traders[0].address(), &config.strategy_denom)
            .u128(),
        config.strategy_denom.clone(),
    );
    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let redeem_1 = coin(
        env.get_balance(&env.traders[1].address(), &config.strategy_denom)
            .u128(),
        config.strategy_denom.clone(),
    );
    env.redeem_fund(&wasm, &vault_addr, redeem_1.clone(), &env.traders[1])
        .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.total_staked_tokens.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(
        contract_base_before,
        contract_base_after.checked_sub(1u128.into()).unwrap(),
    );
}

#[test]
fn test_redeem_end_to_end_with_repayment() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let trader_base_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);

    // Two users deposit
    {
        let deposit = coin(100_000_000, BASE_DENOM);
        env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
            .unwrap();

        let deposit_1 = coin(200_000_000, BASE_DENOM);
        env.deposit_fund(&wasm, &vault_addr, &[deposit_1.clone()], &env.traders[1])
            .unwrap();
    }

    // Controller withdraws
    {
        let withdraw_amount = coin(150_000_000, BASE_DENOM);
        env.withdraw_strategy(
            &wasm,
            &strategy_addr,
            vec![withdraw_amount],
            &env.controller,
        )
        .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(300_000_000),
            total_withdrawn_tokens: coins(150_000_000, BASE_DENOM),
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Controller repays - with profit
    {
        let profit = coin(50_000_000, BASE_DENOM);
        // send funds to the controller
        env.send(&strategy_addr, profit.into(), &env.traders[5]);

        let repay_amount = coins(200_000_000, BASE_DENOM);
        env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
            .unwrap();
    }

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
            .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(200_000_000),
            total_withdrawn_tokens: coins(0u128, BASE_DENOM),
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    let expected_profit = Uint128::new(16_611_296); // approx 1/3 of the profit
    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);

    assert_approx_eq!(
        trader_base_before.checked_add(expected_profit).unwrap(),
        trader_base_after,
        "2"
    );
}

#[test]
fn test_redeem_end_to_end_with_partial_repayment() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let trader_base_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);

    // Two users deposit
    {
        let deposit = coin(100_000_000, BASE_DENOM);
        env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
            .unwrap();

        let deposit_1 = coin(200_000_000, BASE_DENOM);
        env.deposit_fund(&wasm, &vault_addr, &[deposit_1.clone()], &env.traders[1])
            .unwrap();
    }

    // Controller withdraws
    {
        let withdraw_amount = coin(150_000_000, BASE_DENOM);
        env.withdraw_strategy(
            &wasm,
            &strategy_addr,
            vec![withdraw_amount],
            &env.controller,
        )
        .unwrap();

        let latest_block_time = env.app.get_block_timestamp();
        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(300_000_000),
            total_withdrawn_tokens: coins(150_000_000, BASE_DENOM),
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Controller repays - with no profit
    {
        let repay_amount = coins(100_000_000, BASE_DENOM);
        env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
            .unwrap();
    }

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
            .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(200_000_000),
            total_withdrawn_tokens: coins(50_000_000, BASE_DENOM), // Shortfall in repayment
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    let expected_loss = Uint128::new(16_611_296); // approx 1/3 of the loss
    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);

    assert_approx_eq!(
        trader_base_before.checked_sub(expected_loss).unwrap(),
        trader_base_after,
        "2"
    );
}

#[test]
fn test_redeem_end_to_end_with_repayment_multiple_denom() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let trader_base_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_before = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);

    // Two users deposit
    {
        let deposit_base = coin(100_000_000, BASE_DENOM);
        let deposit_quote = coin(50_000_000, QUOTE_DENOM);
        env.deposit_fund(
            &wasm,
            &vault_addr,
            &[deposit_base.clone(), deposit_quote.clone()],
            &env.traders[0],
        )
        .unwrap();

        let deposit_base_1 = coin(200_000_000, BASE_DENOM);
        env.deposit_fund(
            &wasm,
            &vault_addr,
            &[deposit_base_1.clone()],
            &env.traders[1],
        )
        .unwrap();
    }

    // Controller withdraws
    {
        let withdraw_amount_base = coin(150_000_000, BASE_DENOM);
        let withdraw_amount_quote = coin(50_000_000, QUOTE_DENOM);
        env.withdraw_strategy(
            &wasm,
            &strategy_addr,
            vec![withdraw_amount_base.clone(), withdraw_amount_quote.clone()],
            &env.controller,
        )
        .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(362_500_000),
            total_withdrawn_tokens: vec![withdraw_amount_quote, withdraw_amount_base],
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Controller repays - with profit
    {
        let profit = coin(50_000_000, BASE_DENOM);
        // send funds to the controller
        env.send(&strategy_addr, profit.into(), &env.traders[5]);

        let repay_amount_base = coin(200_000_000, BASE_DENOM);
        let repay_amount_quote = coin(50_000_000, QUOTE_DENOM);
        env.repay_strategy(
            &wasm,
            &strategy_addr,
            vec![repay_amount_base, repay_amount_quote],
            None,
            &env.controller,
        )
        .unwrap();
    }

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
            .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(200_000_000),
            total_withdrawn_tokens: vec![coin(0u128, QUOTE_DENOM), coin(0u128, BASE_DENOM)],
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    let expected_profit = Uint128::new(16_611_296); // approx 1/3 of the profit
    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_after = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);

    assert_approx_eq!(
        trader_base_before.checked_add(expected_profit).unwrap(),
        trader_base_after,
        "2"
    );

    assert_approx_eq!(trader_quote_before, trader_quote_after, "2");
}

#[test]
fn test_redeem_from_second_user() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.total_staked_tokens, deposit.amount);

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);

    // send funds to the trader 1
    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());
    env.send(
        &env.traders[1].address(),
        redeem.clone().into(),
        &env.traders[0],
    );

    let trader_strategy_before = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(!trader_strategy_before.is_zero());

    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[1])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_approx_eq!(contract_base_before, contract_base_after, "1");
}

#[test]
fn test_redeem_from_second_user_multiple_times() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = None;

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.total_staked_tokens, deposit.amount);

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);

    // send funds to the trader 1
    let send = coin(
        trader_strategy_before.u128().checked_div(2).unwrap(),
        config.strategy_denom.clone(),
    );
    env.send(
        &env.traders[1].address(),
        send.clone().into(),
        &env.traders[0],
    );

    let trader_strategy_before = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());

    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[0])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let trader_strategy_before = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());

    env.redeem_fund(&wasm, &vault_addr, redeem.clone(), &env.traders[1])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_approx_eq!(contract_base_before, contract_base_after, "1");
}

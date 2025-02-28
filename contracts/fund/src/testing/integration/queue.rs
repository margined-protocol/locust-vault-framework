use cosmwasm_std::{coin, coins, Uint128};
use interface::fund::{Redemption, StateResponse};
use neutron_std::types::osmosis::tokenfactory::WhitelistedHook;
use neutron_test_tube::{Account, Module, Wasm};
use testing::setup::{TestEnv, BASE_DENOM, QUOTE_DENOM};

pub const DUST: Uint128 = Uint128::one();

#[test]
fn test_queue() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 1);
    assert_eq!(
        pending_redemption[0],
        Redemption {
            user: env.traders[0].address(),
            amount: redeem.amount,
            timestamp: timestamp.seconds(),
        }
    );

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    env.withdraw_fund(&wasm, &vault_addr, vec![deposit.clone()], &env.controller)
        .unwrap();

    env.repay_queue_fund(&wasm, &vault_addr, None, None, &[deposit], &env.controller)
        .unwrap();

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_ne!(contract_base_before, contract_base_after);

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert!(pending_redemption.is_empty());
}

#[test]
fn test_queue_twice() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 1);
    assert_eq!(
        pending_redemption[0],
        Redemption {
            user: env.traders[0].address(),
            amount: redeem.amount,
            timestamp: timestamp.seconds(),
        }
    );

    let redeem = coin(
        trader_strategy_before.u128() / 2u128,
        config.strategy_denom.clone(),
    );
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 1);
    assert_eq!(
        pending_redemption[0],
        Redemption {
            user: env.traders[0].address(),
            amount: trader_strategy_before,
            timestamp: timestamp.seconds(),
        }
    );

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    env.withdraw_fund(&wasm, &vault_addr, vec![deposit.clone()], &env.controller)
        .unwrap();

    env.repay_queue_fund(&wasm, &vault_addr, None, None, &[deposit], &env.controller)
        .unwrap();

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_ne!(contract_base_before, contract_base_after);

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert!(pending_redemption.is_empty());
}

#[test]
fn test_queue_multiple_users() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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

    let trader_1_strategy_before =
        env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(!trader_1_strategy_before.is_zero());

    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let redeem_1 = coin(
        trader_1_strategy_before.u128(),
        config.strategy_denom.clone(),
    );
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem_1.clone()], &env.traders[1])
        .unwrap();
    let timestamp_1 = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 2);
    assert_eq!(
        pending_redemption,
        vec![
            Redemption {
                user: env.traders[0].address(),
                amount: redeem.amount,
                timestamp: timestamp.seconds(),
            },
            Redemption {
                user: env.traders[1].address(),
                amount: redeem_1.amount,
                timestamp: timestamp_1.seconds(),
            },
        ]
    );

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let trader_1_strategy_after =
        env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_1_strategy_after.is_zero());

    let total_deposit = coin(
        deposit.amount.checked_add(deposit_1.amount).unwrap().u128(),
        BASE_DENOM,
    );
    env.withdraw_fund(
        &wasm,
        &vault_addr,
        vec![total_deposit.clone()],
        &env.controller,
    )
    .unwrap();

    env.repay_queue_fund(
        &wasm,
        &vault_addr,
        None,
        None,
        &[total_deposit],
        &env.controller,
    )
    .unwrap();

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.total_staked_tokens.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(
        contract_base_before.checked_add(DUST).unwrap(),
        contract_base_after
    );
}

#[test]
fn test_queue_end_to_end_with_repayment() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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
        env.withdraw_fund(&wasm, &vault_addr, vec![withdraw_amount], &env.controller)
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

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
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
        env.send(&env.controller.address(), profit.into(), &env.traders[5]);

        let repay_amount = coins(200_000_000, BASE_DENOM);
        env.repay_queue_fund(
            &wasm,
            &vault_addr,
            None,
            None,
            &repay_amount,
            &env.controller,
        )
        .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(200_000_000),
            total_withdrawn_tokens: coins(0, BASE_DENOM),
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Claim redemption - trader 0
    env.claim_redemption(&wasm, &redemption_addr, None, &env.traders[0])
        .unwrap();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert!(pending_redemption.is_empty());

    let expected_profit = Uint128::new(16_611_296); // approx 1/3 of the profit
    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);

    assert_eq!(
        trader_base_before.checked_add(expected_profit).unwrap(),
        trader_base_after,
    );
}

#[test]
fn test_queue_end_to_end_with_partial_repayment() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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
        env.withdraw_fund(&wasm, &vault_addr, vec![withdraw_amount], &env.controller)
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

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
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
        env.repay_queue_fund(
            &wasm,
            &vault_addr,
            None,
            None,
            &repay_amount,
            &env.controller,
        )
        .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(200_000_000),
            total_withdrawn_tokens: coins(50_000_000, BASE_DENOM),
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Claim redemption - trader 0
    env.claim_redemption(&wasm, &redemption_addr, None, &env.traders[0])
        .unwrap();

    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    assert_eq!(trader_base_before, trader_base_after,);
}

#[test]
fn test_queue_end_to_end_with_repayment_multiple_denom() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);
    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    let block_time = env.app.get_block_timestamp();

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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
            total_staked_tokens: Uint128::new(425_000_000),
            total_withdrawn_tokens: vec![withdraw_amount_quote, withdraw_amount_base],
            last_pause: block_time,
            last_claim: latest_block_time,
            pending_management_fees: vec![],
        };

        let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
        assert_eq!(state, expected_state);
    }

    // Trader redeems
    {
        let redeem = coin(
            env.get_balance(&env.traders[0].address(), &config.strategy_denom)
                .u128(),
            config.strategy_denom.clone(),
        );
        env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
            .unwrap();
        let latest_block_time = env.app.get_block_timestamp();

        let expected_state = StateResponse {
            is_open: true,
            is_paused: false,
            total_staked_tokens: Uint128::new(425_000_000),
            total_withdrawn_tokens: vec![
                coin(50_000_000u128, QUOTE_DENOM),
                coin(150_000_000u128, BASE_DENOM),
            ],
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
        env.repay_queue_strategy(
            &wasm,
            &strategy_addr,
            vec![repay_amount_base, repay_amount_quote],
            None,
            &env.controller,
        )
        .unwrap();
    }

    // Claim redemption - trader 0
    env.claim_redemption(&wasm, &redemption_addr, None, &env.traders[0])
        .unwrap();

    let expected_base_share = Uint128::new(44_105_572);
    let expected_quote_share = Uint128::new(29_472_141);
    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_after = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);

    assert_eq!(
        trader_base_before.checked_add(expected_base_share).unwrap(),
        trader_base_after
    );

    assert_eq!(
        trader_quote_before
            .checked_sub(expected_quote_share)
            .unwrap(),
        trader_quote_after
    );
}

#[test]
fn test_queue_from_second_user() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.whitelist_hooks(vec![WhitelistedHook {
        code_id: 2,
        denom_creator: vault_addr.to_string(),
    }]);

    env.register_sudo_fund(&wasm, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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

    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[1])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 1);
    assert_eq!(
        pending_redemption[0],
        Redemption {
            user: env.traders[1].address(),
            amount: redeem.amount,
            timestamp: timestamp.seconds(),
        }
    );

    let trader_strategy_after = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    env.withdraw_fund(&wasm, &vault_addr, vec![deposit.clone()], &env.controller)
        .unwrap();

    env.repay_queue_fund(&wasm, &vault_addr, None, None, &[deposit], &env.controller)
        .unwrap();

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(
        contract_base_before.checked_add(DUST).unwrap(),
        contract_base_after
    );
}

#[test]
fn test_queue_from_second_user_multiple_times() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.whitelist_hooks(vec![WhitelistedHook {
        code_id: 2,
        denom_creator: vault_addr.to_string(),
    }]);

    env.register_sudo_fund(&wasm, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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

    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let trader_strategy_before = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());

    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[1])
        .unwrap();

    let trader_strategy_after = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 2);

    let trader_strategy_after = env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    env.withdraw_fund(&wasm, &vault_addr, vec![deposit.clone()], &env.controller)
        .unwrap();

    env.repay_queue_fund(&wasm, &vault_addr, None, None, &[deposit], &env.controller)
        .unwrap();

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(
        contract_base_before.checked_add(DUST).unwrap(),
        contract_base_after.checked_sub(DUST).unwrap()
    );

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert!(pending_redemption.is_empty());
}

#[test]
fn test_queue_insufficient_funds_then_complete() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.token1 = None;
    msg.redemption_contract = redemption_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.whitelist_fund(
        &wasm,
        &redemption_addr,
        vault_addr.to_string(),
        "Some metadata".to_string(),
        &env.signer,
    )
    .unwrap();

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

    let trader_1_strategy_before =
        env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(!trader_1_strategy_before.is_zero());

    let redeem = coin(trader_strategy_before.u128(), config.strategy_denom.clone());
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem.clone()], &env.traders[0])
        .unwrap();
    let timestamp = env.app.get_block_timestamp();

    let redeem_1 = coin(
        trader_1_strategy_before.u128(),
        config.strategy_denom.clone(),
    );
    env.create_redemption_fund(&wasm, &vault_addr, &[redeem_1.clone()], &env.traders[1])
        .unwrap();
    let timestamp_1 = env.app.get_block_timestamp();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 2);
    assert_eq!(
        pending_redemption,
        vec![
            Redemption {
                user: env.traders[0].address(),
                amount: redeem.amount,
                timestamp: timestamp.seconds(),
            },
            Redemption {
                user: env.traders[1].address(),
                amount: redeem_1.amount,
                timestamp: timestamp_1.seconds(),
            },
        ]
    );

    let trader_strategy_after = env.get_balance(&env.traders[0].address(), &config.strategy_denom);
    assert!(trader_strategy_after.is_zero());

    let trader_1_strategy_after =
        env.get_balance(&env.traders[1].address(), &config.strategy_denom);
    assert!(trader_1_strategy_after.is_zero());

    let total_deposit = coin(
        deposit.amount.checked_add(deposit_1.amount).unwrap().u128(),
        BASE_DENOM,
    );
    env.withdraw_fund(
        &wasm,
        &vault_addr,
        vec![total_deposit.clone()],
        &env.controller,
    )
    .unwrap();

    // First pay back insufficient funds
    let repayment_insufficient = coin(25_000_000u128, BASE_DENOM);
    env.repay_queue_fund(
        &wasm,
        &vault_addr,
        None,
        None,
        &[repayment_insufficient],
        &env.controller,
    )
    .unwrap();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 2);
    assert_eq!(
        pending_redemption,
        vec![
            Redemption {
                user: env.traders[0].address(),
                amount: redeem.amount,
                timestamp: timestamp.seconds(),
            },
            Redemption {
                user: env.traders[1].address(),
                amount: redeem_1.amount,
                timestamp: timestamp_1.seconds(),
            },
        ]
    );

    // repay first user
    let repayment_sufficient = coin(75_000_000u128, BASE_DENOM);
    env.repay_queue_fund(
        &wasm,
        &vault_addr,
        None,
        None,
        &[repayment_sufficient],
        &env.controller,
    )
    .unwrap();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert_eq!(pending_redemption.len(), 1);
    assert_eq!(
        pending_redemption,
        vec![Redemption {
            user: env.traders[1].address(),
            amount: redeem_1.amount,
            timestamp: timestamp_1.seconds(),
        },]
    );

    // repay second user
    let repayment_sufficient = coin(200_000_000u128, BASE_DENOM);
    env.repay_queue_fund(
        &wasm,
        &vault_addr,
        None,
        None,
        &[repayment_sufficient],
        &env.controller,
    )
    .unwrap();

    let pending_redemption = env
        .query_pending_redemptions_fund(&wasm, &vault_addr, None)
        .unwrap();
    assert!(pending_redemption.is_empty());

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.total_staked_tokens.is_zero());

    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(
        contract_base_before.checked_add(DUST).unwrap(),
        contract_base_after
    );
}

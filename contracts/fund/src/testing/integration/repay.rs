use cosmwasm_std::{coin, coins, Decimal, Uint128};
use interface::fund::StateResponse;
use neutron_test_tube::{Account, Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, GAS_DENOM, QUOTE_DENOM},
    utils::assert_err,
};
use vaultenator::errors::ContractError;

#[test]
fn test_repay() {
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

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let strategy_base_before = env.get_balance(&strategy_addr, BASE_DENOM);

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    let repay_amount = coins(withdraw_amount.into(), BASE_DENOM);
    env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);

    assert_eq!(strategy_base_before, strategy_base_after);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: deposit.amount,
        total_withdrawn_tokens: coins(0u128, BASE_DENOM),
        last_pause: block_time,
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_partial_repay_multiple_denom() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.performance_fee_rate = Decimal::percent(10);

    let mut update_msg = env.default_fund_update_msg();
    update_msg.estimate_cycle_profit = Some(Decimal::percent(10));

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let base = coin(100_000_000, BASE_DENOM);
    let quote = coin(50_000_000, QUOTE_DENOM);
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[base.clone(), quote.clone()],
        &env.traders[0],
    )
    .unwrap();

    let strategy_token_balance = env.get_balance(&env.traders[0].address(), &config.strategy_denom);

    let total_supply = env.get_total_supply(&config.strategy_denom);
    assert!(strategy_token_balance < total_supply);

    let expected_share_before = vec![
        coin(100_382_262u128, BASE_DENOM),
        coin(49_694_189u128, QUOTE_DENOM), // we have a small loss because initial liquidity was provided single sided
    ];
    let share_before = env
        .query_estimate_vault_assets_fund(&wasm, &vault_addr, strategy_token_balance)
        .unwrap();
    assert_eq!(share_before, expected_share_before);

    let withdraw_amount_base = base.amount.checked_div(2u128.into()).unwrap();
    let withdraw_amount_quote = quote.amount.checked_div(2u128.into()).unwrap();

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        vec![
            coin(withdraw_amount_base.into(), BASE_DENOM),
            coin(withdraw_amount_quote.into(), QUOTE_DENOM),
        ],
        &env.controller,
    )
    .unwrap();

    let strategy_base_mid = env.get_balance(&strategy_addr, BASE_DENOM);
    let strategy_quote_mid = env.get_balance(&strategy_addr, QUOTE_DENOM);

    assert_eq!(withdraw_amount_base, strategy_base_mid);
    assert_eq!(withdraw_amount_quote, strategy_quote_mid);

    let repay_amount_base = coin(
        withdraw_amount_base
            .checked_add(Uint128::from(10_000_000u128))
            .unwrap()
            .into(),
        BASE_DENOM,
    );
    // send funds to the controller
    env.send(
        &strategy_addr,
        repay_amount_base.clone().into(),
        &env.traders[0],
    );

    let repay_amount_quote = coin(
        withdraw_amount_quote
            .checked_add(Uint128::from(5_000_000u128))
            .unwrap()
            .into(),
        QUOTE_DENOM,
    );
    // send funds to the controller
    env.send(
        &strategy_addr,
        repay_amount_quote.clone().into(),
        &env.traders[0],
    );

    env.repay_strategy(
        &wasm,
        &strategy_addr,
        vec![repay_amount_base, repay_amount_quote],
        None,
        &env.controller,
    )
    .unwrap();

    let expected_share_post_withdraw = vec![
        coin(109_327_217u128, BASE_DENOM),
        coin(54_166_666u128, QUOTE_DENOM), // we have a small loss because initial liquidity was provided single sided
    ];
    let share_post_withdraw = env
        .query_estimate_vault_assets_fund(&wasm, &vault_addr, strategy_token_balance)
        .unwrap();
    assert_eq!(share_post_withdraw, expected_share_post_withdraw);

    // just to be safe
    assert!(share_post_withdraw[0].amount > share_before[0].amount);
    assert!(share_post_withdraw[1].amount > share_before[1].amount);
}

#[test]
fn test_repay_alt_denom() {
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

    let base = coin(100_000_000, BASE_DENOM);
    let quote = coin(50_000_000, QUOTE_DENOM);
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[base.clone(), quote.clone()],
        &env.traders[0],
    )
    .unwrap();

    let strategy_base_before = env.get_balance(&strategy_addr, BASE_DENOM);
    let strategy_quote_before = env.get_balance(&strategy_addr, QUOTE_DENOM);

    let withdraw_amount_base = base.amount.checked_div(2u128.into()).unwrap();
    let withdraw_amount_quote = quote.amount.checked_div(2u128.into()).unwrap();

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        vec![
            coin(withdraw_amount_base.into(), BASE_DENOM),
            coin(withdraw_amount_quote.into(), QUOTE_DENOM),
        ],
        &env.controller,
    )
    .unwrap();

    let strategy_base_mid = env.get_balance(&strategy_addr, BASE_DENOM);
    let strategy_quote_mid = env.get_balance(&strategy_addr, QUOTE_DENOM);

    assert_eq!(withdraw_amount_base, strategy_base_mid);
    assert_eq!(withdraw_amount_quote, strategy_quote_mid);

    let repay_amount = coins(withdraw_amount_base.into(), BASE_DENOM);
    env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);

    assert_eq!(strategy_base_before, strategy_base_after);

    let repay_amount = coins(withdraw_amount_quote.into(), QUOTE_DENOM);
    env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap();

    let strategy_quote_after = env.get_balance(&strategy_addr, BASE_DENOM);

    assert_eq!(strategy_quote_before, strategy_quote_after);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: Uint128::from(162_500_000u128),
        total_withdrawn_tokens: vec![coin(0u128, BASE_DENOM), coin(0u128, QUOTE_DENOM)],
        last_pause: block_time,
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_repay_alt_denom_single_transaction() {
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

    let base = coin(100_000_000, BASE_DENOM);
    let quote = coin(50_000_000, QUOTE_DENOM);
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[base.clone(), quote.clone()],
        &env.traders[0],
    )
    .unwrap();

    let strategy_base_before = env.get_balance(&strategy_addr, BASE_DENOM);
    let strategy_quote_before = env.get_balance(&strategy_addr, QUOTE_DENOM);

    let withdraw_amount_base = base.amount.checked_div(2u128.into()).unwrap();
    let withdraw_amount_quote = quote.amount.checked_div(2u128.into()).unwrap();

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        vec![
            coin(withdraw_amount_base.into(), BASE_DENOM),
            coin(withdraw_amount_quote.into(), QUOTE_DENOM),
        ],
        &env.controller,
    )
    .unwrap();

    let repay_amount = &[
        coin(withdraw_amount_base.into(), BASE_DENOM),
        coin(withdraw_amount_quote.into(), QUOTE_DENOM),
    ];
    env.repay_strategy(
        &wasm,
        &strategy_addr,
        repay_amount.into(),
        None,
        &env.controller,
    )
    .unwrap();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);

    assert_eq!(strategy_base_before, strategy_base_after);

    let strategy_quote_after = env.get_balance(&strategy_addr, BASE_DENOM);

    assert_eq!(strategy_quote_before, strategy_quote_after);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: Uint128::from(162_500_000u128),
        total_withdrawn_tokens: vec![coin(0u128, BASE_DENOM), coin(0u128, QUOTE_DENOM)],
        last_pause: block_time,
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_repay_post_withdraw_twice() {
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

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let withdraw_amount_1 = deposit.amount.checked_div(4u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount_1.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    let withdraw_amount_2 = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount_2.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    let repay_amount = coins(withdraw_amount_1.into(), BASE_DENOM);
    env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap();

    let repay_amount = coins(withdraw_amount_2.into(), BASE_DENOM);
    env.repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap();

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: Uint128::from(10_000_000u128),
        total_withdrawn_tokens: coins(0u128, BASE_DENOM),
        last_pause: block_time,
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_fail_repay_unauthorized() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    let repay_amount = coins(withdraw_amount.into(), BASE_DENOM);
    let err = env
        .repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.signer)
        .unwrap_err();

    assert_err(err, ContractError::Unauthorized {});
}

#[test]
fn test_fail_repay_incorrect_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    // send incorrect funds to the controller
    let profit = coin(50_000_000, GAS_DENOM);
    env.send(&strategy_addr, profit.into(), &env.traders[5]);

    let repay_amount = coins(withdraw_amount.into(), GAS_DENOM);
    let err = env
        .repay_strategy(&wasm, &strategy_addr, repay_amount, None, &env.controller)
        .unwrap_err();

    assert_err(err, "failed to execute message; message index: 0: dispatch: submessages: Generic error: Failed to retrieve token deposits for two denoms");
}

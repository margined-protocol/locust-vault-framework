use cosmwasm_std::{coin, coins, Decimal, StdError, Uint128};
use interface::fund::StateResponse;
use neutron_test_tube::{Module, Wasm};
use std::str::FromStr;
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::assert_err,
};
use vaultenator::errors::ContractError;

#[test]
fn test_withdraw() {
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

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();
    let latest_block_time = env.app.get_block_timestamp();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);
    assert_eq!(strategy_base_after, withdraw_amount);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: deposit.amount,
        total_withdrawn_tokens: coins(withdraw_amount.into(), BASE_DENOM),
        last_pause: block_time,
        last_claim: latest_block_time,
        pending_management_fees: vec![],
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_withdraw_twice() {
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
    let latest_block_time = env.app.get_block_timestamp();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);
    assert_eq!(
        strategy_base_after,
        withdraw_amount_1.checked_add(withdraw_amount_2).unwrap()
    );

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: deposit.amount,
        total_withdrawn_tokens: coins(
            withdraw_amount_1
                .checked_add(withdraw_amount_2)
                .unwrap()
                .into(),
            BASE_DENOM,
        ),
        last_pause: block_time,
        last_claim: latest_block_time,
        pending_management_fees: vec![],
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_withdraw_with_float() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.float = Decimal::from_str("0.05").unwrap();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let base_deposit = coin(10_000_000, BASE_DENOM);
    let quote_deposit = coin(5_000_000, QUOTE_DENOM);
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[base_deposit.clone(), quote_deposit.clone()],
        &env.traders[0],
    )
    .unwrap();

    let vault_base_before = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(vault_base_before, Uint128::from(11_000_000u128));
    let vault_quote_before = env.get_balance(&vault_addr, QUOTE_DENOM);
    assert_eq!(vault_quote_before, Uint128::from(5_000_000u128));

    let strategy_base_before = env.get_balance(&strategy_addr, BASE_DENOM);
    assert!(strategy_base_before.is_zero());
    let strategy_quote_before = env.get_balance(&strategy_addr, QUOTE_DENOM);
    assert!(strategy_quote_before.is_zero());

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        vec![
            coin(vault_base_before.u128(), BASE_DENOM),
            coin(vault_quote_before.u128(), QUOTE_DENOM),
        ],
        &env.controller,
    )
    .unwrap();

    let vault_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    assert_eq!(vault_base_after, Uint128::from(550_000u128));
    let vault_quote_after = env.get_balance(&vault_addr, QUOTE_DENOM);
    assert_eq!(vault_quote_after, Uint128::from(250_000u128));

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);
    assert_eq!(strategy_base_after, Uint128::from(10_450_000u128));
    let strategy_quote_after = env.get_balance(&strategy_addr, QUOTE_DENOM);
    assert_eq!(strategy_quote_after, Uint128::from(4_750_000u128));

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        vec![
            coin(vault_base_after.u128(), BASE_DENOM),
            coin(vault_quote_after.u128(), QUOTE_DENOM),
        ],
        &env.controller,
    )
    .unwrap();
}

#[test]
fn test_fail_withdraw_not_controller() {
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

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();

    let err = env
        .withdraw_strategy(
            &wasm,
            &strategy_addr,
            coins(withdraw_amount.into(), BASE_DENOM),
            &env.signer,
        )
        .unwrap_err();

    assert_err(err, ContractError::Unauthorized {});
}

#[test]
fn test_fail_withdraw_duplicate_denom() {
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

    let deposit = coin(10_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();

    let err = env
        .withdraw_strategy(
            &wasm,
            &strategy_addr,
            vec![
                coin(withdraw_amount.into(), BASE_DENOM),
                coin(withdraw_amount.into(), BASE_DENOM),
            ],
            &env.controller,
        )
        .unwrap_err();
    assert_err(
        err,
        ContractError::Std(StdError::generic_err("Duplicate denom found: ubase")),
    );
}

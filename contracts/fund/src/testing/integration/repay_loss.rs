use cosmwasm_std::{coin, coins, SignedDecimal, Uint128};
use interface::fund::StateResponse;
use neutron_test_tube::{Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM},
    utils::assert_err,
};

#[test]
fn test_repay_loss() {
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
    assert!(strategy_base_before.is_zero());

    let withdraw_amount = deposit.amount.checked_div(2u128.into()).unwrap();
    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(withdraw_amount.into(), BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    let repay_amount = coins(
        withdraw_amount.checked_div(2u128.into()).unwrap().into(),
        BASE_DENOM,
    );
    env.repay_strategy(
        &wasm,
        &strategy_addr,
        repay_amount,
        Some(SignedDecimal::percent(-50)),
        &env.controller,
    )
    .unwrap();
    let latest_block_time = env.app.get_block_timestamp();

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);
    assert_eq!(Uint128::from(2_500_000u128), strategy_base_after);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: deposit.amount,
        total_withdrawn_tokens: coins(1_250_000u128, BASE_DENOM),
        last_pause: block_time,
        last_claim: latest_block_time,
        pending_management_fees: vec![],
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
}

#[test]
fn test_fail_repay_loss_more_than_one_hundred_percent() {
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
    let profit = coin(50_000_000, BASE_DENOM);
    env.send(&strategy_addr, profit.into(), &env.traders[5]);

    let repay_amount = coins(withdraw_amount.into(), BASE_DENOM);
    let err = env
        .repay_strategy(
            &wasm,
            &strategy_addr,
            repay_amount,
            Some(SignedDecimal::percent(-110)),
            &env.controller,
        )
        .unwrap_err();
    assert_err(err, "Generic error: Profit percentage must be between -100% and 100%: execute wasm contract failed");
}

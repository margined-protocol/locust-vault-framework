use cosmwasm_std::{coin, coins};
use interface::fund::StateResponse;
use osmosis_test_tube::{Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM},
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

    let strategy_base_after = env.get_balance(&strategy_addr, BASE_DENOM);
    assert_eq!(strategy_base_after, withdraw_amount);

    let expected_state = StateResponse {
        is_open: true,
        is_paused: false,
        total_staked_tokens: deposit.amount,
        total_withdrawn_tokens: coins(withdraw_amount.into(), BASE_DENOM),
        last_pause: block_time,
        last_claim: block_time,
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
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);
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

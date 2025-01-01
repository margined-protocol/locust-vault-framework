use crate::errors::ContractError;

use cosmwasm_std::coin;
use neutron_test_tube::{Module, Wasm};
use testing::{
    deployment::get_default_instantiation_msg,
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::assert_err,
};

#[test]
fn test_queue() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr =
        env.deploy_strategy_contract(&wasm, Some(get_default_instantiation_msg(&env)));

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    // fund vault
    let base_amount = 1_000_000_000u128;
    let quote_amount = 6_666_666u128;
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[
            coin(base_amount, BASE_DENOM),
            coin(quote_amount, QUOTE_DENOM),
        ],
        &env.traders[0],
    )
    .unwrap();

    let vault_base_balance_before = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    let tokens_to_withdraw = vec![
        coin(base_amount, BASE_DENOM),
        coin(quote_amount, QUOTE_DENOM),
    ];

    env.withdraw_strategy(&wasm, &strategy_addr, tokens_to_withdraw, &env.controller)
        .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 1_666_666u128;

    let tokens_to_repay = vec![
        coin(repay_base_amount, BASE_DENOM),
        coin(repay_quote_amount, QUOTE_DENOM),
    ];

    env.repay_queue_strategy(
        &wasm,
        &strategy_addr,
        tokens_to_repay,
        None,
        &env.controller,
    )
    .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 5_000_000u128;

    let tokens_to_repay = vec![
        coin(repay_base_amount, BASE_DENOM),
        coin(repay_quote_amount, QUOTE_DENOM),
    ];

    env.repay_queue_strategy(
        &wasm,
        &strategy_addr,
        tokens_to_repay,
        None,
        &env.controller,
    )
    .unwrap();

    let vault_base_balance_after = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_after = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert_eq!(vault_base_balance_before, vault_base_balance_after);
    assert_eq!(vault_quote_balance_before, vault_quote_balance_after);

    let contract_base_balance_after = env.get_balance(&strategy_addr, BASE_DENOM);
    let contract_quote_balance_after = env.get_balance(&strategy_addr, QUOTE_DENOM);

    assert!(contract_base_balance_after.is_zero());
    assert!(contract_quote_balance_after.is_zero());
}

#[test]
fn test_queue_unsorted_coins() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr =
        env.deploy_strategy_contract(&wasm, Some(get_default_instantiation_msg(&env)));

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    // fund vault
    let base_amount = 1_000_000_000u128;
    let quote_amount = 6_666_666u128;
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[
            coin(base_amount, BASE_DENOM),
            coin(quote_amount, QUOTE_DENOM),
        ],
        &env.traders[0],
    )
    .unwrap();

    let vault_base_balance_before = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    let tokens_to_withdraw = vec![
        coin(base_amount, BASE_DENOM),
        coin(quote_amount, QUOTE_DENOM),
    ];

    env.withdraw_strategy(&wasm, &strategy_addr, tokens_to_withdraw, &env.controller)
        .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 1_666_666u128;

    let tokens_to_repay = vec![
        coin(repay_quote_amount, QUOTE_DENOM),
        coin(repay_base_amount, BASE_DENOM),
    ];

    env.repay_queue_strategy(
        &wasm,
        &strategy_addr,
        tokens_to_repay,
        None,
        &env.controller,
    )
    .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 5_000_000u128;

    let tokens_to_repay = vec![
        coin(repay_base_amount, BASE_DENOM),
        coin(repay_quote_amount, QUOTE_DENOM),
    ];

    env.repay_queue_strategy(
        &wasm,
        &strategy_addr,
        tokens_to_repay,
        None,
        &env.controller,
    )
    .unwrap();

    let vault_base_balance_after = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_after = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert_eq!(vault_base_balance_before, vault_base_balance_after);
    assert_eq!(vault_quote_balance_before, vault_quote_balance_after);

    let contract_base_balance_after = env.get_balance(&strategy_addr, BASE_DENOM);
    let contract_quote_balance_after = env.get_balance(&strategy_addr, QUOTE_DENOM);

    assert!(contract_base_balance_after.is_zero());
    assert!(contract_quote_balance_after.is_zero());
}

#[test]
fn test_fail_repay() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let tokens_to_repay = vec![coin(1, BASE_DENOM)];

    let err = env
        .repay_queue_strategy(&wasm, &strategy_addr, tokens_to_repay, None, &env.signer)
        .unwrap_err();
    assert_err(err, ContractError::Unauthorized {});
}

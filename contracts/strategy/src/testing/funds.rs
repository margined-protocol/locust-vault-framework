use crate::errors::ContractError;

use cosmwasm_std::coin;
use osmosis_test_tube::{
    osmosis_std::types::cosmos::base::v1beta1::Coin as BaseCoin, Module, Wasm,
};
use testing::{
    helpers::get_default_instantiation_msg,
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::assert_err,
};

#[test]
fn test_withdraw() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let vault_addr = env.deploy_mock_vault(&wasm);

    // fund vault
    let base_amount = 1_000_000_000u128;
    let quote_amount = 6_666_666u128;

    env.send(
        &vault_addr,
        BaseCoin {
            amount: base_amount.to_string(),
            denom: BASE_DENOM.to_string(),
        },
        &env.signer,
    );
    env.send(
        &vault_addr,
        BaseCoin {
            amount: quote_amount.to_string(),
            denom: QUOTE_DENOM.to_string(),
        },
        &env.signer,
    );

    let vault_base_balance_before = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert_eq!(base_amount, vault_base_balance_before.into());
    assert_eq!(quote_amount, vault_quote_balance_before.into());

    let msg = get_default_instantiation_msg(&env);

    let contract_addr = env.deploy_strategy_contract(&wasm, Some(msg));
    env.set_vault(&wasm, &contract_addr, vault_addr.clone(), &env.signer)
        .unwrap();

    let tokens_to_withdraw = vec![
        coin(base_amount, BASE_DENOM),
        coin(quote_amount, QUOTE_DENOM),
    ];

    env.withdraw(&wasm, &contract_addr, tokens_to_withdraw, &env.controller)
        .unwrap();

    let vault_base_balance_assert = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_assert = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert!(vault_base_balance_assert.is_zero());
    assert!(vault_quote_balance_assert.is_zero());
}

#[test]
fn test_repay() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let vault_addr = env.deploy_mock_vault(&wasm);

    // fund vault
    let base_amount = 1_000_000_000u128;
    let quote_amount = 6_666_666u128;

    env.send(
        &vault_addr,
        BaseCoin {
            amount: base_amount.to_string(),
            denom: BASE_DENOM.to_string(),
        },
        &env.signer,
    );
    env.send(
        &vault_addr,
        BaseCoin {
            amount: quote_amount.to_string(),
            denom: QUOTE_DENOM.to_string(),
        },
        &env.signer,
    );

    let vault_base_balance_before = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    let msg = get_default_instantiation_msg(&env);

    let contract_addr = env.deploy_strategy_contract(&wasm, Some(msg));
    env.set_vault(&wasm, &contract_addr, vault_addr.clone(), &env.signer)
        .unwrap();

    let tokens_to_withdraw = vec![
        coin(base_amount, BASE_DENOM),
        coin(quote_amount, QUOTE_DENOM),
    ];

    env.withdraw(&wasm, &contract_addr, tokens_to_withdraw, &env.controller)
        .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 1_666_666u128;

    let tokens_to_repay = vec![
        coin(repay_base_amount, BASE_DENOM),
        coin(repay_quote_amount, QUOTE_DENOM),
    ];

    env.repay(
        &wasm,
        &contract_addr,
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

    env.repay(
        &wasm,
        &contract_addr,
        tokens_to_repay,
        None,
        &env.controller,
    )
    .unwrap();

    let vault_base_balance_after = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_after = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert_eq!(vault_base_balance_before, vault_base_balance_after);
    assert_eq!(vault_quote_balance_before, vault_quote_balance_after);

    let contract_base_balance_after = env.get_balance(&contract_addr, BASE_DENOM);
    let contract_quote_balance_after = env.get_balance(&contract_addr, QUOTE_DENOM);

    assert!(contract_base_balance_after.is_zero());
    assert!(contract_quote_balance_after.is_zero());
}

#[test]
fn test_fail_withdraw() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let contract_addr = env.deploy_strategy_contract(&wasm, None);

    let tokens_to_withdraw = vec![coin(1, BASE_DENOM)];

    let err = env
        .withdraw(&wasm, &contract_addr, tokens_to_withdraw, &env.signer)
        .unwrap_err();
    assert_err(err, ContractError::Unauthorized {});
}

#[test]
fn test_fail_repay() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let contract_addr = env.deploy_strategy_contract(&wasm, None);

    let tokens_to_repay = vec![coin(1, BASE_DENOM)];

    let err = env
        .repay(&wasm, &contract_addr, tokens_to_repay, None, &env.signer)
        .unwrap_err();
    assert_err(err, ContractError::Unauthorized {});
}

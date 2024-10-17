use crate::errors::ContractError;

use cosmwasm_std::coin;
use osmosis_test_tube::{Module, Wasm};
use testing::{
    deployment::get_default_instantiation_msg,
    setup::{TestEnv, BASE_DENOM, DEFAULT_LIQUIDITY, QUOTE_DENOM},
    utils::assert_err,
};

#[test]
fn test_withdraw() {
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

    assert_eq!(
        base_amount + DEFAULT_LIQUIDITY,
        vault_base_balance_before.into()
    );
    assert_eq!(quote_amount, vault_quote_balance_before.into());

    let tokens_to_withdraw = vec![
        coin(quote_amount, QUOTE_DENOM),
        coin(base_amount + DEFAULT_LIQUIDITY, BASE_DENOM),
    ];

    env.withdraw_strategy(&wasm, &strategy_addr, tokens_to_withdraw, &env.controller)
        .unwrap();

    let vault_base_balance_assert = env.get_balance(&vault_addr, BASE_DENOM);
    let vault_quote_balance_assert = env.get_balance(&vault_addr, QUOTE_DENOM);

    assert!(vault_base_balance_assert.is_zero());
    assert!(vault_quote_balance_assert.is_zero());
}

#[test]
fn test_fail_withdraw() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let tokens_to_withdraw = vec![coin(1, BASE_DENOM)];

    let err = env
        .withdraw_strategy(&wasm, &strategy_addr, tokens_to_withdraw, &env.signer)
        .unwrap_err();
    assert_err(err, ContractError::Unauthorized {});
}

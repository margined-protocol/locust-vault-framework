use cosmwasm_schema::cw_serde;
use cosmwasm_std::{assert_approx_eq, coin, coins, Uint128};
use osmosis_test_tube::{Account, Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::{assert_err, get_strategy_denom_fund},
};
use vaultenator::errors::ContractError;

#[cw_serde]
enum StrategyContractQueryMsg {
    SpotPrice {},
    TwapPrice { duration: u64 },
}

#[test]
fn test_deposit() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let trader_base_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_before = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);
    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);
    let contract_quote_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    let deposit = coin(100_000_000, BASE_DENOM);
    env.deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();

    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_after = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);
    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    let contract_quote_after = env.get_balance(&vault_addr, QUOTE_DENOM);
    let strategy_denom_minted = env.get_balance(
        &env.traders[0].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(100_000_000);
    assert_eq!(strategy_denom_minted, expected_strategy_denom_minted);

    assert_eq!(
        contract_base_before.checked_add(deposit.amount).unwrap(),
        contract_base_after
    );
    assert_eq!(contract_quote_before, contract_quote_after);
    assert_eq!(
        trader_base_after,
        trader_base_before.checked_sub(deposit.amount).unwrap()
    );
    assert_eq!(trader_quote_after, trader_quote_before);
}

#[test]
fn test_deposit_multiple_token() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();
    msg.token1 = Some(QUOTE_DENOM.to_string());

    let vault_addr = env.deploy_fund_contract(&wasm, msg);
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let trader_base_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_before = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);
    let contract_base_before = env.get_balance(&vault_addr, BASE_DENOM);
    let contract_quote_before = env.get_balance(&vault_addr, QUOTE_DENOM);

    let base = coin(100_000_000, BASE_DENOM);
    let quote = coin(50_000_000, QUOTE_DENOM);
    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[base.clone(), quote.clone()],
        &env.traders[0],
    )
    .unwrap();

    let trader_base_after = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    let trader_quote_after = env.get_balance(&env.traders[0].address(), QUOTE_DENOM);
    let contract_base_after = env.get_balance(&vault_addr, BASE_DENOM);
    let contract_quote_after = env.get_balance(&vault_addr, QUOTE_DENOM);

    let strategy_denom_minted = env.get_balance(
        &env.traders[0].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(140_000_000);
    assert_approx_eq!(strategy_denom_minted, expected_strategy_denom_minted, "1");

    assert_eq!(
        contract_base_before.checked_add(base.amount).unwrap(),
        contract_base_after
    );
    assert_eq!(
        contract_quote_before.checked_add(quote.amount).unwrap(),
        contract_quote_after
    );
    assert_eq!(
        trader_base_after,
        trader_base_before.checked_sub(base.amount).unwrap()
    );
    assert_eq!(
        trader_quote_after,
        trader_quote_before.checked_sub(quote.amount).unwrap()
    );
}

#[test]
fn test_deposit_twice() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(100_000_000, BASE_DENOM).clone()],
        &env.traders[0],
    )
    .unwrap();

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(1_000_000, BASE_DENOM).clone()],
        &env.traders[0],
    )
    .unwrap();

    let strategy_denom_minted = env.get_balance(
        &env.traders[0].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(100_999_999);
    assert_eq!(strategy_denom_minted, expected_strategy_denom_minted);
}

#[test]
fn test_deposit_post_withdrawal() {
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

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(100_000_000, BASE_DENOM).clone()],
        &env.traders[0],
    )
    .unwrap();

    env.withdraw_strategy(
        &wasm,
        &strategy_addr,
        coins(100_000_000, BASE_DENOM),
        &env.controller,
    )
    .unwrap();

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(1_000_000, BASE_DENOM).clone()],
        &env.traders[0],
    )
    .unwrap();

    let strategy_denom_minted = env.get_balance(
        &env.traders[0].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(100_999_999);
    assert_eq!(strategy_denom_minted, expected_strategy_denom_minted);
}

#[test]
fn test_deposit_multiple_users() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(100_000_000, BASE_DENOM).clone()],
        &env.traders[0],
    )
    .unwrap();

    env.deposit_fund(
        &wasm,
        &vault_addr,
        &[coin(200_000_000, BASE_DENOM).clone()],
        &env.traders[1],
    )
    .unwrap();

    let strategy_denom_minted = env.get_balance(
        &env.traders[0].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(100_000_000);
    assert_approx_eq!(strategy_denom_minted, expected_strategy_denom_minted, "1");

    let strategy_denom_minted = env.get_balance(
        &env.traders[1].address(),
        &get_strategy_denom_fund(&vault_addr),
    );

    let expected_strategy_denom_minted = Uint128::new(200_000_000);
    assert_approx_eq!(strategy_denom_minted, expected_strategy_denom_minted, "1");
}

#[test]
fn test_deposit_exceeding_strategy_cap() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let expected_strategy_cap = 10_000_000_000_000u128;

    // Sanity check the strategy_cap we are testing
    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(config.strategy_cap, Uint128::new(expected_strategy_cap));

    let deposit = coin(expected_strategy_cap + 1, BASE_DENOM);
    let res_err = env
        .deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap_err();

    assert_err(res_err, ContractError::StrategyCapExceeded {});
}

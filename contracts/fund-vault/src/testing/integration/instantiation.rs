use crate::reply::INITIAL_TOKEN_SUPPLY;

use cosmwasm_std::{coins, Decimal, StdError, Uint128};
use interface::fund::{ConfigResponse, InstantiateMsg, StateResponse};
use osmosis_test_tube::{Account, Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM, STRATEGY_CAP},
    utils::assert_err,
};
use vaultenator::errors::ContractError;

#[test]
fn test_instantiation() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let msg = InstantiateMsg {
        admin: env.signer.address().to_string(),
        controller: env.signer.address().to_string(),
        treasury: env.treasury.address().to_string(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        strategy_cap: STRATEGY_CAP,
        performance_fee_rate: Decimal::zero(),
        float: Decimal::zero(),
        vault_type: "fund".to_string(),
    };

    let deposit_amount = 1_000_000u128;
    let vault_addr = env
        .instantiate_contract(
            &wasm,
            &msg,
            coins(deposit_amount, BASE_DENOM),
            &env.signer,
            "fund_vault",
        )
        .unwrap()
        .data
        .address;

    let block_time = env.app.get_block_timestamp();

    let expected_config = ConfigResponse {
        admin: env.signer.address().to_string(),
        strategy_cap: STRATEGY_CAP,
        float: Decimal::zero(),
        strategy_denom: format!("factory/{}/fund-vault", vault_addr),
        controller: env.signer.address(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        treasury: env.treasury.address(),
        performance_fee_rate: Decimal::zero(),
        estimate_cycle_profit: None,
        vault_type: "fund".to_string(),
    };

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(config, expected_config);

    let expected_state = StateResponse {
        is_open: false,
        is_paused: false,
        total_staked_tokens: Default::default(),
        total_withdrawn_tokens: Default::default(),
        last_pause: block_time,
        last_claim: block_time,
    };

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state, expected_state);

    let signer_strategy_denom_balance =
        env.get_balance(&env.signer.address(), &config.strategy_denom.clone());

    let contract_base_denom_balance = env.get_balance(&vault_addr, &config.token0);
    let contract_quote_denom_balance = env.get_balance(&vault_addr, QUOTE_DENOM);

    let strategy_denom_total_supply = env.get_total_supply(&config.strategy_denom);

    assert_eq!(strategy_denom_total_supply, INITIAL_TOKEN_SUPPLY);
    assert_eq!(signer_strategy_denom_balance, Uint128::zero()); // Initial token supply is PoL

    assert_eq!(contract_base_denom_balance, Uint128::from(deposit_amount));
    assert_eq!(contract_quote_denom_balance, Uint128::zero());
    assert_eq!(state.total_staked_tokens, Uint128::zero());
    assert_eq!(state.total_withdrawn_tokens, vec![]);
}

#[test]
fn test_fail_instantiation_strategy_cap_zero() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let msg = InstantiateMsg {
        admin: env.signer.address().to_string(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        controller: env.signer.address().to_string(),
        strategy_cap: Uint128::zero(),
        performance_fee_rate: Decimal::zero(),
        float: Decimal::zero(),
        treasury: env.treasury.address().to_string(),
        vault_type: "fund".to_string(),
    };

    let err = env
        .instantiate_contract(&wasm, &msg, vec![], &env.signer, "fund_vault")
        .unwrap_err();
    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Strategy cap must be greater than zero",
        )),
    );
}

#[test]
fn test_fail_instantiation_performance_fee_rate_invalid() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let msg = InstantiateMsg {
        admin: env.signer.address().to_string(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        controller: env.signer.address().to_string(),
        strategy_cap: STRATEGY_CAP,
        performance_fee_rate: Decimal::percent(21),
        float: Decimal::zero(),
        treasury: env.treasury.address().to_string(),
        vault_type: "fund".to_string(),
    };

    let err = env
        .instantiate_contract(&wasm, &msg, vec![], &env.signer, "fund_vault")
        .unwrap_err();
    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Performance fee rate must be less or equal to twenty percent",
        )),
    );
}

#[test]
fn test_fail_instantiation_float_invalid() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let msg = InstantiateMsg {
        admin: env.signer.address().to_string(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        controller: env.signer.address().to_string(),
        strategy_cap: STRATEGY_CAP,
        performance_fee_rate: Decimal::percent(10),
        float: Decimal::percent(11),
        treasury: env.treasury.address().to_string(),
        vault_type: "fund".to_string(),
    };

    let err = env
        .instantiate_contract(&wasm, &msg, vec![], &env.signer, "fund_vault")
        .unwrap_err();
    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Float must be less or equal to ten percent",
        )),
    );
}

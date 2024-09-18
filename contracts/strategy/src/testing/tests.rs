use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    errors::ContractError,
};

use cosmwasm_std::{coin, Decimal, StdError};
use interface::strategy::{ConfigResponse, InstantiateMsg, PoolInfo};
use osmosis_test_tube::{
    osmosis_std::types::{
        cosmos::base::v1beta1::Coin as BaseCoin,
        osmosis::concentratedliquidity::v1beta1::MsgCreatePosition,
    },
    Account, Module, Wasm,
};
use std::str::FromStr;
use testing::{
    helpers::get_default_instantiation_msg,
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::{assert_err, store_code},
};

#[test]
fn test_instantiation() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "strategy").unwrap();

    let contract_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                controller: env.controller.address(),
                vault: env.signer.address(),
                token0: BASE_DENOM.to_string(),
                token1: None,
                grants: vec![MsgCreatePosition::TYPE_URL.to_string()],
                pool_info: PoolInfo::Osmosis {
                    id: 1,
                    token0: BASE_DENOM.to_string(),
                    token1: QUOTE_DENOM.to_string(),
                },
            },
            None,
            Some("strategy-contract"),
            &[],
            &env.signer,
        )
        .unwrap()
        .data
        .address;

    let config = env.query_config(&wasm, &contract_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            controller: env.controller.address(),
            vault: env.signer.address(),
            token0: BASE_DENOM.to_string(),
            token1: None,
            pool_info: PoolInfo::Osmosis {
                id: 1,
                token0: BASE_DENOM.to_string(),
                token1: QUOTE_DENOM.to_string(),
            },
            grants: vec![MsgCreatePosition::TYPE_URL.to_string()],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    )
}

#[test]
fn test_fail_instantiation() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "strategy").unwrap();

    let err = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                controller: env.controller.address(),
                vault: env.signer.address(),
                token0: BASE_DENOM.to_string(),
                token1: None,
                grants: vec![],
                pool_info: PoolInfo::Osmosis {
                    id: 1,
                    token0: BASE_DENOM.to_string(),
                    token1: QUOTE_DENOM.to_string(),
                },
            },
            None,
            Some("strategy-contract"),
            &[],
            &env.signer,
        )
        .unwrap_err();

    assert_err(
        err,
        ContractError::Std(StdError::generic_err("Grants must be non-empty")),
    );
}

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

    let mut msg = get_default_instantiation_msg(&env);
    msg.vault = vault_addr.to_string();

    let contract_addr = env.deploy_strategy_contract(&wasm, Some(msg));

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

    let mut msg = get_default_instantiation_msg(&env);
    msg.vault = vault_addr.to_string();

    let contract_addr = env.deploy_strategy_contract(&wasm, Some(msg));

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

    env.repay(&wasm, &contract_addr, tokens_to_repay, &env.controller)
        .unwrap();

    let repay_base_amount = 500_000_000u128;
    let repay_quote_amount = 5_000_000u128;

    let tokens_to_repay = vec![
        coin(repay_base_amount, BASE_DENOM),
        coin(repay_quote_amount, QUOTE_DENOM),
    ];

    env.repay(&wasm, &contract_addr, tokens_to_repay, &env.controller)
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
        .repay(&wasm, &contract_addr, tokens_to_repay, &env.signer)
        .unwrap_err();
    assert_err(err, ContractError::Unauthorized {});
}

#[test]
fn test_query_grants() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let contract_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_grants = vec![MsgCreatePosition::TYPE_URL.to_string()];

    let actual_grants = env.query_grants(&wasm, &contract_addr).unwrap();
    assert_eq!(expected_grants, actual_grants);
}

#[test]
fn test_query_spot_price() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let contract_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_spot_price = Decimal::from_str("1.250000000000000001").unwrap();

    let actual_spot_price = env.query_spot_price(&wasm, &contract_addr).unwrap();
    assert_eq!(expected_spot_price, actual_spot_price);
}

#[test]
fn test_query_twap_price() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let contract_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_twap_price = Decimal::from_str("1.250000000000000001").unwrap();

    let actual_twap_price = env
        .query_twap_price(&wasm, &contract_addr, 3600u64)
        .unwrap();
    assert_eq!(expected_twap_price, actual_twap_price);
}

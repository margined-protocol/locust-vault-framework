use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    reply::INITIAL_TOKEN_SUPPLY,
};

use cosmwasm_std::{coin, StdError, Uint128};
use cw_vault_standard::{VaultStandardInfoResponse, VaultStandardQueryMsg};
use interface::fund::QueryMsg;
use neutron_test_tube::{Account, Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::assert_err,
};

#[test]
fn query_vault_standard_info() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    let vault_info = wasm
        .query::<QueryMsg, VaultStandardInfoResponse>(
            &vault_addr,
            &VaultStandardQueryMsg::VaultStandardInfo {},
        )
        .unwrap();
    assert_eq!(vault_info.version, 1.to_string());
    assert_eq!(vault_info.extensions.len(), 2);
    let expected_extensions = vec!["lockup".to_string(), "force-unlock".to_string()];
    assert_eq!(vault_info.extensions, expected_extensions);
}

#[test]
fn query_info() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();
    let res = env.query_info_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(res.base_token, config.token0);
    assert_eq!(res.vault_token, config.strategy_denom);
}

#[test]
fn query_preview_deposit() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let amount = Uint128::from(100_000u128);

    env.query_preview_deposit_fund(&wasm, &vault_addr, amount)
        .unwrap_err();
}

#[test]
fn query_preview_redeem() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let amount = Uint128::from(100_000u128);

    env.query_preview_redeem_fund(&wasm, &vault_addr, amount)
        .unwrap_err();
}

#[test]
fn query_total_vault_token_supply() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    // Instantation opens a position and issues INITIAL_TOKEN_SUPPLY
    let total_vault_token_supply = env
        .query_total_vault_token_supply_fund(&wasm, &vault_addr)
        .unwrap();
    assert_eq!(total_vault_token_supply, INITIAL_TOKEN_SUPPLY);
}

#[test]
fn query_convert_to_shares() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let amount = Uint128::from(100_000u128);

    env.query_convert_to_shares_fund(&wasm, &vault_addr, amount)
        .unwrap_err();
}

#[test]
fn query_convert_to_assets() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    let total_supply = env
        .query_total_vault_token_supply_fund(&wasm, &vault_addr)
        .unwrap();

    let expected_amount = Uint128::from(1_250_000u128);

    let convert_to_assets = env
        .query_convert_to_assets_fund(&wasm, &vault_addr, total_supply)
        .unwrap();
    assert_eq!(convert_to_assets, expected_amount);
}

#[test]
fn query_estimate_vault_assets() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    let total_supply = env
        .query_total_vault_token_supply_fund(&wasm, &vault_addr)
        .unwrap();

    let expected_amount = Uint128::from(1_000_000u128);

    let convert_to_assets = env
        .query_estimate_vault_assets_fund(&wasm, &vault_addr, total_supply)
        .unwrap();
    assert_eq!(
        convert_to_assets,
        [
            coin(expected_amount.into(), BASE_DENOM),
            coin(0u128, QUOTE_DENOM)
        ]
    );
}

#[test]
fn query_fail_convert_to_assets_amount_greater_than_total_supply() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    let total_supply = env
        .query_total_vault_token_supply_fund(&wasm, &vault_addr)
        .unwrap();

    let err = env
        .query_convert_to_assets_fund(&wasm, &vault_addr, total_supply + Uint128::one())
        .unwrap_err();
    assert_err(err, StdError::generic_err("Amount exceeds total supply"));
}

// Extension functions
#[test]
fn query_owner() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let owner = env.query_owner_fund(&wasm, &vault_addr).unwrap();

    assert_eq!(owner.to_string(), env.signer.address());
}

#[test]
fn query_config_fund() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    let expected_strategy_denom = format!("factory/{}/{}", vault_addr, env!("CARGO_PKG_NAME"),);

    assert_eq!(config.strategy_denom, expected_strategy_denom);
    assert_eq!(config.token0, BASE_DENOM.to_string());
}

#[test]
fn query_state() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();

    let timestamp = &env.app.get_block_timestamp();

    assert!(!state.is_open);
    assert!(!state.is_paused);
    assert_eq!(state.last_pause, *timestamp);
}

#[test]
fn query_version() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    let version = env.query_version_fund(&wasm, &vault_addr).unwrap();

    assert_eq!(version.name, format!("crates.io:{CONTRACT_NAME}"));
    assert_eq!(version.version, CONTRACT_VERSION.to_string());
}

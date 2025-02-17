use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    errors::ContractError,
};

use cosmwasm_std::StdError;
use interface::redemption::{ConfigResponse, FundInfo, InstantiateMsg};
use neutron_test_tube::{Account, Module, Wasm};
use testing::{
    setup::TestEnv,
    utils::{assert_err, store_code},
};

#[test]
fn test_instantiation() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "redemption").unwrap();

    let redemption_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                whitelisted_funds: vec![FundInfo {
                    address: env.traders[0].address(),
                    metadata: "test fund".to_string(),
                }],
            },
            None,
            Some("redemption-contract"),
            &[],
            &env.signer,
        )
        .unwrap()
        .data
        .address;

    let config = env
        .query_config_redemption(&wasm, &redemption_addr)
        .unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            whitelisted_funds: vec![FundInfo {
                address: env.traders[0].address(),
                metadata: "test fund".to_string(),
            }],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    );
}

#[test]
fn test_fail_instantiation_duplicate_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "redemption").unwrap();

    let err = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                whitelisted_funds: vec![
                    FundInfo {
                        address: env.traders[0].address(),
                        metadata: "test fund".to_string(),
                    },
                    FundInfo {
                        address: env.traders[0].address(),
                        metadata: "duplicate fund".to_string(),
                    },
                ],
            },
            None,
            Some("redemption-contract"),
            &[],
            &env.signer,
        )
        .unwrap_err();

    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Duplicate fund contracts are not allowed",
        )),
    );
}

#[test]
fn test_fail_instantiation_invalid_fund_address() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "redemption").unwrap();

    let err = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                whitelisted_funds: vec![FundInfo {
                    address: "invalid/address".to_string(),
                    metadata: "test fund".to_string(),
                }],
            },
            None,
            Some("redemption-contract"),
            &[],
            &env.signer,
        )
        .unwrap_err();
    println!("err: {}", err);

    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "addr_validate errored: decoding bech32 failed",
        )),
    );
}

#[test]
fn test_fail_instantiation_invalid_metadata_length() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "redemption").unwrap();

    let err = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                whitelisted_funds: vec![FundInfo {
                    address: env.traders[0].address(),
                    metadata: "a".repeat(256),
                }],
            },
            None,
            Some("redemption-contract"),
            &[],
            &env.signer,
        )
        .unwrap_err();

    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Metadata length exceeds maximum of 255 characters",
        )),
    );
}

#[test]
fn test_fail_instantiation_too_many_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let code_id = store_code(&wasm, &env.signer, "redemption").unwrap();

    // Create more than MAX_WHITELISTED_FUNDS funds
    let mut funds = Vec::new();
    for i in 0..101 {
        funds.push(FundInfo {
            address: env.traders[i % env.traders.len()].address(),
            metadata: format!("test fund {}", i),
        });
    }

    let err = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                whitelisted_funds: funds,
            },
            None,
            Some("redemption-contract"),
            &[],
            &env.signer,
        )
        .unwrap_err();

    assert_err(
        err,
        ContractError::Std(StdError::generic_err(
            "Number of whitelisted funds exceeds maximum of 100",
        )),
    );
}

use crate::errors::ContractError;

use cosmwasm_std::{coin, StdError};
use interface::redemption::FundInfo;
use neutron_test_tube::{Account, Module, Wasm};
use testing::{setup::TestEnv, utils::assert_err};

#[test]
fn test_update_config() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let new_fund = FundInfo {
        address: env.traders[0].address(),
        metadata: "new fund".to_string(),
    };

    // Test adding a fund
    env.update_config(
        &wasm,
        &redemption_addr,
        Some(new_fund.clone()),
        None,
        &env.signer,
    )
    .unwrap();

    let config = env
        .query_config_redemption(&wasm, &redemption_addr)
        .unwrap();
    assert_eq!(config.whitelisted_funds.len(), 2);
    assert!(config.whitelisted_funds.contains(&new_fund));

    // Test removing a fund
    env.update_config(&wasm, &redemption_addr, None, Some(new_fund), &env.signer)
        .unwrap();

    let config = env
        .query_config_redemption(&wasm, &redemption_addr)
        .unwrap();
    assert_eq!(config.whitelisted_funds.len(), 1);
}

#[test]
fn test_fail_update_config_too_many_funds() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    // Add funds until we hit the limit
    for i in 1..100 {
        let address = env
            .app
            .init_account(&[coin(1000000000000000000u128, "untr")])
            .unwrap();

        let new_fund = FundInfo {
            address: address.address().to_string(),
            metadata: format!("test fund {}", i),
        };

        env.update_config(&wasm, &redemption_addr, Some(new_fund), None, &env.signer)
            .unwrap();
    }

    // Try to add one more fund
    let err = env
        .update_config(
            &wasm,
            &redemption_addr,
            Some(FundInfo {
                address: env.traders[0].address(),
                metadata: "one too many".to_string(),
            }),
            None,
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

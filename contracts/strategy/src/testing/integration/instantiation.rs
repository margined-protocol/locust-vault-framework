use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    errors::ContractError,
};

use cosmwasm_std::StdError;
use interface::strategy::{ConfigResponse, InstantiateMsg, PoolInfo};
use neutron_test_tube::{
    neutron_std::types::neutron::dex::MsgPlaceLimitOrder as DefaultMsg, Account, Module, Wasm,
};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::{assert_err, store_code},
};

#[test]
fn test_instantiation() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.signer, "strategy").unwrap();

    let strategy_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                admin: env.signer.address(),
                controller: env.controller.address(),
                token0: BASE_DENOM.to_string(),
                token1: None,
                grants: vec![DefaultMsg::TYPE_URL.to_string()],
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

    let config = env.query_config_strategy(&wasm, &strategy_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            controller: env.controller.address(),
            vault: None,
            token0: BASE_DENOM.to_string(),
            token1: None,
            pool_info: PoolInfo::Osmosis {
                id: 1,
                token0: BASE_DENOM.to_string(),
                token1: QUOTE_DENOM.to_string(),
            },
            grants: vec![DefaultMsg::TYPE_URL.to_string()],
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
                admin: env.signer.address(),
                controller: env.controller.address(),
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

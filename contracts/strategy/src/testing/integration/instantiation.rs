use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    errors::ContractError,
};

use cosmwasm_std::{coin, StdError};
use interface::strategy::{ConfigResponse, InstantiateMsg, PoolInfo};
use neutron_test_tube::{
    neutron_std::{
        shim::Any,
        types::{
            cosmos::{
                authz::v1beta1::{GrantAuthorization, QueryGranterGrantsRequest},
                bank::v1beta1::SendAuthorization,
            },
            neutron::dex::MsgPlaceLimitOrder as DefaultMsg,
        },
    },
    Account, Authz, Module, Wasm,
};
use prost::Message;
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
                send_authorization: None,
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
            send_authorization: None,
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    )
}

#[test]
fn test_instantiation_with_allow_list() {
    let env = TestEnv::new();

    let authz = Authz::new(&env.app);
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
                send_authorization: Some(SendAuthorization {
                    spend_limit: vec![coin(1000000000000000000u128, BASE_DENOM).into()],
                    allow_list: vec![env.traders[0].address()],
                }),
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

    let res = authz
        .query_granter_grants(&QueryGranterGrantsRequest {
            granter: strategy_addr.to_string(),
            pagination: None,
        })
        .unwrap();

    assert_eq!(res.grants.len(), 2);
    assert_eq!(
        res.grants[0],
        GrantAuthorization {
            granter: strategy_addr.clone(),
            grantee: env.controller.address(),
            authorization: Some(Any {
                type_url: SendAuthorization::TYPE_URL.to_string(),
                value: SendAuthorization {
                    spend_limit: vec![coin(1000000000000000000u128, BASE_DENOM).into()],
                    allow_list: vec![env.traders[0].address()],
                }
                .encode_to_vec(),
            }),
            expiration: None,
        }
    );
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
                send_authorization: None,
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
fn test_fail_instantiation_duplicate_grant() {
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
                grants: vec![
                    DefaultMsg::TYPE_URL.to_string(),
                    DefaultMsg::TYPE_URL.to_string(),
                ],
                send_authorization: None,
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
        ContractError::Std(StdError::generic_err("Duplicate grants are not allowed")),
    );
}

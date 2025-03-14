use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_std::coin;
use interface::strategy::{ConfigResponse, InstantiateMsg, PoolInfo};
use neutron_test_tube::{
    neutron_std::{
        shim::Any,
        types::{
            cosmos::{
                authz::v1beta1::MsgExec,
                bank::v1beta1::{MsgSend, SendAuthorization},
            },
            neutron::dex::{MsgCancelLimitOrder as SecondMsg, MsgPlaceLimitOrder as DefaultMsg},
        },
    },
    Account, Authz, Module, Wasm,
};
use prost::Message;
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::store_code,
};

#[test]
fn test_set_vault() {
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

    env.set_vault_strategy(
        &wasm,
        &strategy_addr,
        &env.controller.address(),
        &env.signer,
    )
    .unwrap();

    let config = env.query_config_strategy(&wasm, &strategy_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            controller: env.controller.address(),
            vault: Some(env.controller.address()),
            token0: BASE_DENOM.to_string(),
            token1: None,
            send_authorization: None,
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
fn test_set_grants() {
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

    env.set_grants_strategy(
        &wasm,
        &strategy_addr,
        vec![SecondMsg::TYPE_URL.to_string()],
        &env.signer,
    )
    .unwrap();

    let config = env.query_config_strategy(&wasm, &strategy_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            controller: env.controller.address(),
            vault: None,
            token0: BASE_DENOM.to_string(),
            token1: None,
            send_authorization: None,
            pool_info: PoolInfo::Osmosis {
                id: 1,
                token0: BASE_DENOM.to_string(),
                token1: QUOTE_DENOM.to_string(),
            },
            grants: vec![SecondMsg::TYPE_URL.to_string()],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    )
}

#[test]
fn test_update_config() {
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

    env.update_config_strategy(
        &wasm,
        &strategy_addr,
        Some(vec![SecondMsg::TYPE_URL.to_string()]),
        None,
        &env.signer,
    )
    .unwrap();

    let config = env.query_config_strategy(&wasm, &strategy_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            controller: env.controller.address(),
            vault: None,
            token0: BASE_DENOM.to_string(),
            token1: None,
            send_authorization: None,
            pool_info: PoolInfo::Osmosis {
                id: 1,
                token0: BASE_DENOM.to_string(),
                token1: QUOTE_DENOM.to_string(),
            },
            grants: vec![SecondMsg::TYPE_URL.to_string()],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    );

    env.update_config_strategy(
        &wasm,
        &strategy_addr,
        None,
        Some(env.traders[5].address()),
        &env.signer,
    )
    .unwrap();

    let config = env.query_config_strategy(&wasm, &strategy_addr).unwrap();

    assert_eq!(
        config,
        ConfigResponse {
            admin: env.signer.address(),
            controller: env.traders[5].address(),
            vault: None,
            token0: BASE_DENOM.to_string(),
            token1: None,
            send_authorization: None,
            pool_info: PoolInfo::Osmosis {
                id: 1,
                token0: BASE_DENOM.to_string(),
                token1: QUOTE_DENOM.to_string(),
            },
            grants: vec![SecondMsg::TYPE_URL.to_string()],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: CONTRACT_VERSION.to_string(),
        }
    )
}

#[test]
fn test_send_authorization() {
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
                    spend_limit: vec![coin(100u128, BASE_DENOM).into()],
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

    env.send(
        &strategy_addr,
        coin(100u128, BASE_DENOM).into(),
        &env.signer,
    );

    let recipient_balance_before = env.get_balance(&env.traders[0].address(), BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![coin(101u128, BASE_DENOM).into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap_err();

    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[1].address().to_string(),
                        amount: vec![coin(100u128, BASE_DENOM).into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap_err();

    let amount_sent = coin(50u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();

    assert_eq!(
        env.get_balance(&env.traders[0].address(), BASE_DENOM),
        recipient_balance_before + amount_sent.amount
    );

    let second_amount_sent = coin(50u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![second_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();

    assert_eq!(
        env.get_balance(&env.traders[0].address(), BASE_DENOM),
        recipient_balance_before + amount_sent.amount + second_amount_sent.amount
    );

    let third_amount_sent = coin(1u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![third_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap_err();
}

#[test]
fn test_set_authorization() {
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
                    spend_limit: vec![coin(100u128, BASE_DENOM).into()],
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

    env.send(
        &strategy_addr,
        coin(200u128, BASE_DENOM).into(),
        &env.signer,
    );

    let amount_sent = coin(100u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();

    let second_amount_sent = coin(1u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![second_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap_err();

    env.set_send_authorization_strategy(&wasm, &strategy_addr, &env.signer)
        .unwrap();

    let third_amount_sent = coin(100u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![third_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();
}

#[test]
fn test_set_authorization_before_expiration() {
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
                    spend_limit: vec![coin(100u128, BASE_DENOM).into()],
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

    env.send(
        &strategy_addr,
        coin(200u128, BASE_DENOM).into(),
        &env.signer,
    );

    let amount_sent = coin(50u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();

    let second_amount_sent = coin(1u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![second_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();

    env.set_send_authorization_strategy(&wasm, &strategy_addr, &env.signer)
        .unwrap();

    let third_amount_sent = coin(100u128, BASE_DENOM);
    authz
        .exec(
            MsgExec {
                grantee: env.controller.address().to_string(),
                msgs: vec![Any {
                    type_url: MsgSend::TYPE_URL.to_string(),
                    value: MsgSend {
                        from_address: strategy_addr.to_string(),
                        to_address: env.traders[0].address().to_string(),
                        amount: vec![third_amount_sent.clone().into()],
                    }
                    .encode_to_vec(),
                }],
            },
            &env.controller,
        )
        .unwrap();
}

use cosmwasm_std::Decimal;
#[cfg(feature = "astroport")]
use interface::strategy::PoolInfo;
use osmosis_test_tube::{
    osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePosition, Module, Wasm,
};
use std::str::FromStr;
use testing::setup::TestEnv;
#[cfg(feature = "astroport")]
use testing::{
    deployment::get_default_instantiation_msg,
    setup::{BASE_DENOM, QUOTE_DENOM},
};

#[test]
fn test_query_grants() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_grants = vec![MsgCreatePosition::TYPE_URL.to_string()];

    let actual_grants = env.query_grants_strategy(&wasm, &strategy_addr).unwrap();
    assert_eq!(expected_grants, actual_grants);
}

#[cfg(feature = "osmosis")]
#[test]
fn test_query_spot_price_osmosis() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_spot_price = Decimal::from_str("1.250000000000000001").unwrap();

    let actual_spot_price = env
        .query_spot_price_strategy(&wasm, &strategy_addr)
        .unwrap();
    assert_eq!(expected_spot_price, actual_spot_price);
}

#[cfg(feature = "astroport")]
#[test]
fn test_query_spot_price_astroport() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let astro_addr = env.deploy_mock_astro(&wasm);
    env.set_astro_price_strategy(
        &wasm,
        &astro_addr,
        Decimal::from_str("1.25").unwrap(),
        &env.signer,
    )
    .unwrap();

    let mut msg = get_default_instantiation_msg(&env);
    msg.pool_info = PoolInfo::Astroport {
        pool_address: astro_addr.clone(),
        token0: BASE_DENOM.to_string(),
        token1: QUOTE_DENOM.to_string(),
    };

    let strategy_addr = env.deploy_strategy_contract(&wasm, Some(msg));

    let expected_spot_price = Decimal::from_str("1.25").unwrap();

    let actual_spot_price = env
        .query_spot_price_strategy(&wasm, &strategy_addr)
        .unwrap();
    assert_eq!(expected_spot_price, actual_spot_price);
}

#[cfg(feature = "osmosis")]
#[test]
fn test_query_twap_price_osmosis() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let expected_twap_price = Decimal::from_str("1.250000000000000001").unwrap();

    let actual_twap_price = env
        .query_twap_price_strategy(&wasm, &strategy_addr, 3600u64)
        .unwrap();
    assert_eq!(expected_twap_price, actual_twap_price);
}

#[cfg(feature = "astroport")]
#[test]
fn test_query_twap_price_astroport() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let astro_addr = env.deploy_mock_astro(&wasm);
    env.set_astro_price_strategy(
        &wasm,
        &astro_addr,
        Decimal::from_str("1.25").unwrap(),
        &env.signer,
    )
    .unwrap();

    let mut msg = get_default_instantiation_msg(&env);
    msg.pool_info = PoolInfo::Astroport {
        pool_address: astro_addr.clone(),
        token0: BASE_DENOM.to_string(),
        token1: QUOTE_DENOM.to_string(),
    };

    let strategy_addr = env.deploy_strategy_contract(&wasm, Some(msg));

    let expected_twap_price = Decimal::from_str("1.25").unwrap();

    let actual_twap_price = env
        .query_twap_price_strategy(&wasm, &strategy_addr, 3600u64)
        .unwrap();
    assert_eq!(expected_twap_price, actual_twap_price);
}

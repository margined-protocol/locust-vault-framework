use cosmwasm_std::coins;
use neutron_test_tube::{Account, Module, Wasm};
use testing::setup::{TestEnv, BASE_DENOM};

use interface::redemption::PendingRedemption;

#[test]
fn test_query_redemptions() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    // Create and send multiple redemptions
    let mut redemption1 = PendingRedemption {
        user: env.signer.address(),
        funds: coins(100, BASE_DENOM),
        timestamp: 1000,
        source: env.fund.address(),
    };

    let mut redemption2 = PendingRedemption {
        user: env.signer.address(),
        funds: coins(200, BASE_DENOM),
        timestamp: 2000,
        source: env.fund.address(),
    };

    env.send_redemption(
        &wasm,
        &redemption_addr,
        redemption1.clone(),
        &redemption1.funds,
        &env.fund,
    )
    .unwrap();

    redemption1.timestamp = env.app.get_block_time_seconds() as u64;

    env.send_redemption(
        &wasm,
        &redemption_addr,
        redemption2.clone(),
        &redemption2.funds,
        &env.fund,
    )
    .unwrap();

    redemption2.timestamp = env.app.get_block_time_seconds() as u64;

    // Test query_all_redemptions
    let all_redemptions = env
        .query_all_redemptions(&wasm, &redemption_addr, None, Some(10))
        .unwrap();
    assert_eq!(all_redemptions.len(), 2);
    assert_eq!(all_redemptions[0], redemption1);
    assert_eq!(all_redemptions[1], redemption2);

    // Test query_redemptions for specific user
    let user_redemptions = env
        .query_redemptions(&wasm, &redemption_addr, env.signer.address(), Some(10))
        .unwrap();
    assert_eq!(user_redemptions.len(), 2);
    assert_eq!(user_redemptions[0], redemption1);
    assert_eq!(user_redemptions[1], redemption2);

    // Test pagination
    let paginated = env
        .query_all_redemptions(&wasm, &redemption_addr, None, Some(1))
        .unwrap();
    assert_eq!(paginated.len(), 1);
    assert_eq!(paginated[0], redemption1);
}

#[test]
fn test_query_config() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let redemption_addr = env.deploy_redemption_contract(&wasm, None);

    let config = env
        .query_config_redemption(&wasm, &redemption_addr)
        .unwrap();
    assert_eq!(config.admin, env.signer.address());
    assert_eq!(config.whitelisted_funds.len(), 1);
}

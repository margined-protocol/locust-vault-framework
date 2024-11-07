use cosmwasm_std::Uint128;
use interface::fund::UpdateConfig;
use neutron_test_tube::{Module, Wasm};
use testing::setup::{TestEnv, BASE_DENOM};

#[test]
fn update_config() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();

    assert_eq!(config.token0, BASE_DENOM.to_string());

    let updated_config = UpdateConfig {
        strategy_cap: Some(100u128.into()),
        float: None,
        controller: None,
        performance_fee_rate: None,
        estimate_cycle_profit: None,
        treasury: None,
        instant_withdraw_penalty: None,
        penalty_duration: None,
    };

    env.update_config_fund(&wasm, &vault_addr, &env.signer, updated_config)
        .unwrap();

    let config = env.query_config_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(config.strategy_cap, Uint128::from(100u128));
}

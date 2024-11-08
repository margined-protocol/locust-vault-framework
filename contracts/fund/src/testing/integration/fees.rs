use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Decimal};
use osmosis_test_tube::{Module, Wasm};
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::contains_event_with_attributes,
};

#[cw_serde]
enum StrategyContractQueryMsg {
    SpotPrice {},
    TwapPrice { duration: u64 },
}

pub const HALF_YEAR_IN_SECONDS: u64 = 15_768_000;

#[test]
fn testt_fees() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.management_fee_rate = Decimal::percent(2);
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let deposit = coin(100_000_000, BASE_DENOM);
    let res = env
        .deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some("")
    ));

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let deposit = coin(100_000_000, BASE_DENOM);
    let res = env
        .deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();
    println!("{:#?}", res);
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("1010000{}", BASE_DENOM))
    ));

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let res = env.crank_fund(&wasm, &vault_addr, &env.traders[0]).unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("1999900{}", BASE_DENOM))
    ));
}

#[test]
fn test_fees_multiple_denoms() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.management_fee_rate = Decimal::percent(2);
    msg.controller = strategy_addr.to_string();
    msg.token1 = Some(QUOTE_DENOM.to_string());

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let base = coin(100_000_000, BASE_DENOM);
    let quote = coin(50_000_000, QUOTE_DENOM);
    let res = env
        .deposit_fund(
            &wasm,
            &vault_addr,
            &[base.clone(), quote.clone()],
            &env.traders[0],
        )
        .unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some("")
    ));

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let base = coin(200_000_000, BASE_DENOM);
    let quote = coin(10_000_000, QUOTE_DENOM);
    let res = env
        .deposit_fund(
            &wasm,
            &vault_addr,
            &[base.clone(), quote.clone()],
            &env.traders[0],
        )
        .unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    println!("res: {:#?}", res);
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("1010000{}, 500000{}", BASE_DENOM, QUOTE_DENOM))
    ));

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let res = env.crank_fund(&wasm, &vault_addr, &env.traders[0]).unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("2999900{}, 595000{}", BASE_DENOM, QUOTE_DENOM)),
    ));
}

#[test]
fn test_fees_all_assets_withdrawn() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let strategy_addr = env.deploy_strategy_contract(&wasm, None);

    let mut msg = env.default_fund_instantiation_msg();
    msg.management_fee_rate = Decimal::percent(2);
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let deposit = coin(100_000_000, BASE_DENOM);
    let res = env
        .deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some("")
    ));

    env.withdraw_strategy(&wasm, &strategy_addr, vec![deposit], &env.controller)
        .unwrap();

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let res = env.crank_fund(&wasm, &vault_addr, &env.traders[0]).unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("1000000{}", BASE_DENOM))
    ));

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(
        state.pending_management_fees,
        vec![coin(10_000, BASE_DENOM)]
    );

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let res = env.crank_fund(&wasm, &vault_addr, &env.traders[0]).unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(""),
    ));

    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(
        state.pending_management_fees,
        vec![coin(1010000, BASE_DENOM)]
    );

    env.app.increase_time(HALF_YEAR_IN_SECONDS);

    let deposit = coin(100_000_000, BASE_DENOM);
    let res = env
        .deposit_fund(&wasm, &vault_addr, &[deposit.clone()], &env.traders[0])
        .unwrap();
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fee_type"),
        Some("management")
    ));
    assert!(contains_event_with_attributes(
        &res,
        "fees",
        Some("fees"),
        Some(&format!("2010000{}", BASE_DENOM))
    ));
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert_eq!(state.pending_management_fees, vec![]);
}

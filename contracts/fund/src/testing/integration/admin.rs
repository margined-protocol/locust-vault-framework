use cw_controllers::AdminError;
use neutron_test_tube::{Module, Wasm};
use testing::{
    setup::TestEnv,
    utils::{assert_err, contains_event},
};
use vaultenator::errors::ContractError;

#[test]
fn set_open() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    let res = env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(!state.is_paused);
    assert!(contains_event(&res, "open_contract"))
}

#[test]
fn set_open_already_open() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let res_err = env
        .set_open_fund(&wasm, &vault_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::IsOpen {});
}

#[test]
fn set_open_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    let res_err = env
        .set_open_fund(&wasm, &vault_addr, &env.traders[0])
        .unwrap_err();

    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_pause() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(!state.is_paused);

    let res = env.set_pause_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(state.is_paused);
    assert!(contains_event(&res, "paused"))
}

#[test]
fn set_pause_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(!state.is_paused);

    let res_err = env
        .set_pause_fund(&wasm, &vault_addr, &env.traders[0])
        .unwrap_err();
    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_pause_contract_is_already_paused() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    env.set_pause_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let res_err = env
        .set_pause_fund(&wasm, &vault_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::Paused {});
}

#[test]
fn set_unpause() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());

    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(!state.is_paused);

    env.set_pause_fund(&wasm, &vault_addr, &env.signer).unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(state.is_paused);

    let res = env
        .set_unpause_fund(&wasm, &vault_addr, &env.signer)
        .unwrap();
    let state = env.query_state_fund(&wasm, &vault_addr).unwrap();
    assert!(state.is_open);
    assert!(!state.is_paused);
    assert!(contains_event(&res, "unpaused"))
}

#[test]
fn set_unpause_not_admin() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let res_err = env
        .set_unpause_fund(&wasm, &vault_addr, &env.traders[0])
        .unwrap_err();
    assert_err(res_err, ContractError::Admin(AdminError::NotAdmin {}));
}

#[test]
fn set_unpause_contract_is_already_unpaused() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);
    let vault_addr = env.deploy_fund_contract(&wasm, env.default_fund_instantiation_msg());
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    let res_err = env
        .set_unpause_fund(&wasm, &vault_addr, &env.signer)
        .unwrap_err();

    assert_err(res_err, ContractError::NotPaused {});
}

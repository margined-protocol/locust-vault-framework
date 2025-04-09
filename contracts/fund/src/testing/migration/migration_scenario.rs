use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Decimal, Uint128};
use interface::fund::{
    ConfigResponse, ExtensionQueryMsg, MigrateMsg, QueryMsg, VaultenatorExtensionQueryMsg,
};
use neutron_test_tube::{Account, Module, Runner, Wasm};
use osmosis_std::types::cosmwasm::wasm::v1::{
    MsgMigrateContract, MsgMigrateContractResponse, QueryContractInfoRequest,
    QueryContractInfoResponse,
};
use test_tube::ExecuteResponse;
use testing::{
    setup::{TestEnv, BASE_DENOM, STRATEGY_CAP},
    utils::store_code,
};

#[cw_serde]
pub struct V020InstantiateMsg {
    pub admin: String,               // manages contract configuration
    pub controller: String,          // manages contract balance sheet
    pub treasury: String,            // account fees are paid to
    pub redemption_contract: String, // redemption contract address
    pub strategy_cap: Uint128,       // maximum value of strategy deposits
    pub float: Decimal,              // percentage of balance sheet that can be withdrawn
    pub token0: String,
    pub token1: Option<String>,
    pub management_fee_rate: Decimal,
    pub performance_fee_rate: Decimal,
    pub vault_type: String,
}

#[test]
fn test_migration() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let wasm_byte_code =
        std::fs::read("../../contracts/fund/src/testing/migration/fund-v020.wasm").unwrap();

    let fund_vault_v003 = wasm
        .store_code(&wasm_byte_code, None, &env.signer)
        .unwrap()
        .data
        .code_id;

    let msg = V020InstantiateMsg {
        admin: env.signer.address().to_string(),
        controller: env.signer.address().to_string(),
        redemption_contract: env.controller.address().to_string(),
        treasury: env.treasury.address().to_string(),
        strategy_cap: STRATEGY_CAP,
        float: Decimal::zero(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        management_fee_rate: Decimal::zero(),
        performance_fee_rate: Decimal::zero(),
        vault_type: "fund".to_string(),
    };

    let funds = vec![coin(1_000_000_000, BASE_DENOM)];

    let contract_addr = wasm
        .instantiate(
            fund_vault_v003,
            &msg,
            Some(&env.signer.address()),
            Some("fund-vault"),
            &funds,
            &env.signer,
        )
        .unwrap()
        .data
        .address;
    env.set_open_fund(&wasm, &contract_addr, &env.signer)
        .unwrap();

    // let config = env.query_config(&wasm, &contract_addr).unwrap();
    let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
        VaultenatorExtensionQueryMsg::Config {},
    ));

    let config: ConfigResponse = wasm.query(&contract_addr, &query_msg).unwrap();

    let expected_strategy_denom = format!("factory/{}/fund", contract_addr);

    assert_eq!(config.strategy_denom, expected_strategy_denom);
    assert_eq!(config.token0, BASE_DENOM.to_string());

    let version = env.query_version_fund(&wasm, &contract_addr).unwrap();

    assert_eq!(version.name, "crates.io:fund".to_string());
    assert_eq!(version.version, "0.2.0".to_string());

    let code_id = store_code(&wasm, &env.signer, "fund").unwrap();

    // migrate contract
    let _res: ExecuteResponse<MsgMigrateContractResponse> = env
        .app
        .execute(
            MsgMigrateContract {
                sender: env.signer.address(),
                contract: contract_addr.clone(),
                code_id,
                msg: serde_json_wasm::to_vec(&MigrateMsg {
                    redemption_contract: env.controller.address().to_string(),
                })
                .unwrap(),
            },
            "/cosmwasm.wasm.v1.MsgMigrateContract",
            &env.signer,
        )
        .unwrap();

    // check the code_id
    let res: QueryContractInfoResponse = env
        .app
        .query(
            "/cosmwasm.wasm.v1.Query/ContractInfo",
            &QueryContractInfoRequest {
                address: contract_addr.clone(),
            },
        )
        .unwrap();

    let contract_info = res.contract_info.unwrap();

    assert_eq!(res.address, contract_addr);
    assert_eq!(contract_info.code_id, code_id);
    assert_eq!(contract_info.creator, env.signer.address());
    assert_eq!(contract_info.label, "fund-vault");

    let config = env.query_config_fund(&wasm, &contract_addr).unwrap();
    let expected_strategy_denom = format!("factory/{}/fund", contract_addr);

    assert_eq!(config.strategy_denom, expected_strategy_denom);
    assert_eq!(config.token0, BASE_DENOM.to_string());
    assert_eq!(
        config.redemption_contract,
        env.controller.address().to_string()
    );

    let version = env.query_version_fund(&wasm, &contract_addr).unwrap();

    assert_eq!(version.name, format!("crates.io:{CONTRACT_NAME}"));
    assert_eq!(version.version, CONTRACT_VERSION.to_string());
}

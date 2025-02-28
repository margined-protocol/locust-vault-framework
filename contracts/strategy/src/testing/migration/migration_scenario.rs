use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_std::coin;
use interface::strategy::{ConfigResponse, InstantiateMsg, MigrateMsg, PoolInfo};
// use osmosis_std::types::{
//     cosmwasm::wasm::v1::{
//         MsgMigrateContract, MsgMigrateContractResponse, QueryContractInfoRequest,
//         QueryContractInfoResponse,
//     },
//     osmosis::concentratedliquidity::v1beta1::DefaultMsg as DefaultMsg,
// };
// use osmosis_test_tube::{Account, Module, Runner, Wasm};
use neutron_std::types::{
    cosmwasm::wasm::v1::{
        MsgMigrateContract, MsgMigrateContractResponse, QueryContractInfoRequest,
        QueryContractInfoResponse,
    },
    neutron::dex::MsgPlaceLimitOrder as DefaultMsg,
};
use neutron_test_tube::{Account, Module, Runner, Wasm};
use test_tube::ExecuteResponse;
use testing::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::store_code,
};

#[test]
fn test_migration() {
    let env = TestEnv::new();
    let wasm = Wasm::new(&env.app);

    let wasm_byte_code =
        std::fs::read("../../contracts/strategy/src/testing/migration/strategy-v004.wasm").unwrap();

    let fund_vault_v003 = wasm
        .store_code(&wasm_byte_code, None, &env.signer)
        .unwrap()
        .data
        .code_id;

    let msg = InstantiateMsg {
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
    };

    let funds = vec![coin(1_000_000_000, BASE_DENOM)];

    let strategy_addr = wasm
        .instantiate(
            fund_vault_v003,
            &msg,
            Some(&env.signer.address()),
            Some("strategy"),
            &funds,
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
            grants: vec![DefaultMsg::TYPE_URL.to_string(),],
            name: format!("crates.io:{}", CONTRACT_NAME),
            version: "0.0.4".to_string(),
        }
    );

    let code_id = store_code(&wasm, &env.signer, "strategy").unwrap();

    // migrate contract
    let _res: ExecuteResponse<MsgMigrateContractResponse> = env
        .app
        .execute(
            MsgMigrateContract {
                sender: env.signer.address(),
                contract: strategy_addr.clone(),
                code_id,
                msg: serde_json_wasm::to_vec(&MigrateMsg {}).unwrap(),
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
                address: strategy_addr.clone(),
            },
        )
        .unwrap();

    let contract_info = res.contract_info.unwrap();

    assert_eq!(res.address, strategy_addr);
    assert_eq!(contract_info.code_id, code_id);
    assert_eq!(contract_info.creator, env.signer.address());
    assert_eq!(contract_info.label, "strategy");
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

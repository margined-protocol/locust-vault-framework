use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Decimal, Uint128};
use interface::strategy::{ConfigResponse, InstantiateMsg, MigrateMsg, PoolInfo, QueryMsg};
use neutron_std::types::{
    cosmwasm::wasm::v1::{
        MsgMigrateContract, MsgMigrateContractResponse, QueryContractInfoRequest,
        QueryContractInfoResponse,
    },
    neutron::dex::MsgPlaceLimitOrder,
};
use neutron_test_tube::{Account, Module, Runner, RunnerResult, Wasm};
use test_tube::ExecuteResponse;
use testing::{
    neutron::{NeutronTestEnv, BASE_DENOM, QUOTE_DENOM, STRATEGY_CAP},
    utils::store_code,
};

#[cfg(feature = "slinky")]
#[test]
fn test_slinky() {
    let env = NeutronTestEnv::new();
    // let wasm = Wasm::new(&env.app);

    // let wasm_byte_code = std::fs::read("../../artifacts/strategy-slinky.wasm").unwrap();

    // let code_id = wasm
    //     .store_code(&wasm_byte_code, None, &env.signer)
    //     .unwrap()
    //     .data
    //     .code_id;

    // let msg = InstantiateMsg {
    //     admin: env.signer.address(),
    //     controller: env.controller.address(),
    //     token0: BASE_DENOM.to_string(),
    //     token1: None,
    //     grants: vec![MsgPlaceLimitOrder::TYPE_URL.to_string()],
    //     pool_info: PoolInfo::Slinky {
    //         base: BASE_DENOM.to_string(),
    //         quote: QUOTE_DENOM.to_string(),
    //         timeout: 60u64,
    //     },
    // };

    // let contract_addr = wasm
    //     .instantiate(
    //         code_id,
    //         &msg,
    //         Some(&env.signer.address()),
    //         Some("strategy-contract-slinky"),
    //         &[],
    //         &env.signer,
    //     )
    //     .unwrap()
    //     .data
    //     .address;

    // let query_msg = QueryMsg::Config {};

    // let config: ConfigResponse = wasm.query(&contract_addr, &query_msg).unwrap();

    // assert_eq!(
    //     config,
    //     ConfigResponse {
    //         admin: env.signer.address(),
    //         controller: env.controller.address(),
    //         vault: None,
    //         token0: BASE_DENOM.to_string(),
    //         token1: None,
    //         pool_info: PoolInfo::Osmosis {
    //             id: 1,
    //             token0: BASE_DENOM.to_string(),
    //             token1: QUOTE_DENOM.to_string(),
    //         },
    //         grants: vec![MsgPlaceLimitOrder::TYPE_URL.to_string()],
    //         name: format!("crates.io:{}", CONTRACT_NAME),
    //         version: "0.0.1".to_string(),
    //     }
    // );

    // let query_msg = QueryMsg::SpotPrice {};
    // let spot_price: Decimal = wasm.query(&contract_addr, &query_msg).unwrap();

    // assert_eq!(spot_price, Decimal::zero());
}

use crate::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM},
    utils::store_code,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Uint128};
use interface::strategy::{InstantiateMsg, PoolInfo};
use osmosis_std::types::{
    cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
    cosmwasm::wasm::v1::MsgInstantiateContractResponse,
    osmosis::{
        concentratedliquidity::v1beta1::MsgCreatePosition,
        poolmanager::v1beta1::{PoolRequest, SpotPriceRequest},
    },
};
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::{
        cosmos::bank::v1beta1::{QueryBalanceRequest, QueryTotalSupplyRequest},
        osmosis::concentratedliquidity::v1beta1::Pool,
    },
    Account, Bank, Module, OsmosisTestApp, PoolManager, RunnerExecuteResult, SigningAccount, Wasm,
};
use serde::Serialize;
use std::str::FromStr;

#[cw_serde]
pub struct MockInstantiateMsg {}

pub fn get_default_instantiation_msg(env: &TestEnv) -> InstantiateMsg {
    InstantiateMsg {
        admin: env.signer.address(),
        controller: env.controller.address(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        grants: vec![MsgCreatePosition::TYPE_URL.to_string()],
        pool_info: PoolInfo::Osmosis {
            id: 1,
            token0: BASE_DENOM.to_string(),
            token1: QUOTE_DENOM.to_string(),
        },
    }
}

impl TestEnv {
    pub fn instantiate_contract<M>(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: &M,
        funds: Vec<Coin>,
        signer: &SigningAccount,
        contract_name: &str,
    ) -> RunnerExecuteResult<MsgInstantiateContractResponse>
    where
        M: Serialize,
    {
        let code_id = store_code(wasm, signer, contract_name).unwrap();
        wasm.instantiate(
            code_id,
            msg,
            Some(&self.signer.address()),
            Some("vault"),
            &funds,
            signer,
        )
    }

    pub fn deploy_strategy_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: Option<InstantiateMsg>,
    ) -> String {
        let msg = msg.unwrap_or_else(|| get_default_instantiation_msg(self));

        self.instantiate_contract(wasm, &msg, vec![], &self.signer, "strategy")
            .unwrap()
            .data
            .address
    }

    pub fn deploy_mock_vault(&self, wasm: &Wasm<OsmosisTestApp>) -> String {
        self.instantiate_contract(
            wasm,
            &MockInstantiateMsg {},
            vec![],
            &self.signer,
            "mock_vault",
        )
        .unwrap()
        .data
        .address
    }

    pub fn deploy_mock_astro(&self, wasm: &Wasm<OsmosisTestApp>) -> String {
        self.instantiate_contract(
            wasm,
            &MockInstantiateMsg {},
            vec![],
            &self.signer,
            "mock_astro",
        )
        .unwrap()
        .data
        .address
    }

    pub fn send(&self, to_address: &str, amount: OsmoCoin, sender: &SigningAccount) {
        let bank = Bank::new(&self.app);

        bank.send(
            MsgSend {
                from_address: sender.address(),
                to_address: to_address.to_string(),
                amount: vec![amount],
            },
            sender,
        )
        .unwrap();
    }

    pub fn get_balance(&self, address: &str, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address: address.to_string(),
                denom: denom.to_string(),
            })
            .unwrap();

        match response.balance {
            Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
            None => Uint128::zero(),
        }
    }

    pub fn get_pool(&self, pool_id: u64) -> Pool {
        let poolmanager = PoolManager::new(&self.app);
        let res = poolmanager.query_pool(&PoolRequest { pool_id }).unwrap();
        Pool::decode(res.pool.unwrap().value.as_ref()).unwrap()
    }

    pub fn get_spot_price(
        &self,
        pool_id: u64,
        base_asset_denom: &str,
        quote_asset_denom: &str,
    ) -> Decimal {
        let pm = PoolManager::new(&self.app);

        let res = pm
            .query_spot_price(&SpotPriceRequest {
                pool_id,
                base_asset_denom: base_asset_denom.to_string(),
                quote_asset_denom: quote_asset_denom.to_string(),
            })
            .unwrap()
            .spot_price;

        Decimal::from_str(&res).unwrap()
    }

    pub fn get_total_supply(&self, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_total_supply(&QueryTotalSupplyRequest { pagination: None })
            .unwrap()
            .supply
            .into_iter()
            .find(|coin| coin.denom == denom)
            .unwrap();

        Uint128::from_str(&response.amount).unwrap_or(Uint128::zero())
    }
}

use crate::{
    setup::{TestEnv, BASE_DENOM, DEFAULT_LIQUIDITY, QUOTE_DENOM, STRATEGY_CAP},
    utils::store_code,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal};
use interface::{fund as Fund, strategy as Strategy};
use neutron_std::types::{
    cosmwasm::wasm::v1::MsgInstantiateContractResponse,
    neutron::dex::MsgPlaceLimitOrder as DefaultMsg,
};
use neutron_test_tube::{
    Account, NeutronTestApp as OsmosisTestApp, RunnerExecuteResult, SigningAccount, Wasm,
};
use serde::Serialize;

#[cw_serde]
enum PoolInfo {
    Osmosis {
        id: u64,
        token0: String,
        token1: String,
    },
    Neutron {},
}

#[cw_serde]
struct StrategyContractInstantiateMsg {
    pub admin: String,
    pub controller: String,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub pool_info: PoolInfo,
}

#[cw_serde]
enum StrategyContractExecuteMsg {
    SetVault { vault: String },
}

#[cw_serde]
pub struct MockInstantiateMsg {}

pub fn get_default_instantiation_msg(env: &TestEnv) -> Strategy::InstantiateMsg {
    Strategy::InstantiateMsg {
        admin: env.signer.address(),
        controller: env.controller.address(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        grants: vec![DefaultMsg::TYPE_URL.to_string()],
        pool_info: Strategy::PoolInfo::Slinky {
            base: BASE_DENOM.to_uppercase(),
            quote: QUOTE_DENOM.to_uppercase(),
            timeout: 3_600u64,
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

    pub fn deploy_fund_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: Fund::InstantiateMsg,
    ) -> String {
        let funds = vec![coin(DEFAULT_LIQUIDITY, BASE_DENOM)];

        self.instantiate_contract(wasm, &msg, funds, &self.signer, "fund")
            .unwrap()
            .data
            .address
    }

    pub fn deploy_strategy_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: Option<Strategy::InstantiateMsg>,
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

    pub fn default_fund_instantiation_msg(&self) -> Fund::InstantiateMsg {
        Fund::InstantiateMsg {
            admin: self.signer.address().to_string(),
            controller: self.controller.address().to_string(),
            treasury: self.treasury.address().to_string(),
            strategy_cap: STRATEGY_CAP,
            float: Decimal::zero(),
            token0: BASE_DENOM.to_string(),
            token1: Some(QUOTE_DENOM.to_string()),
            management_fee_rate: Decimal::zero(),
            performance_fee_rate: Decimal::zero(),
            vault_type: "fund".to_string(),
        }
    }

    pub fn default_fund_update_msg(&self) -> Fund::UpdateConfig {
        Fund::UpdateConfig {
            strategy_cap: None,
            float: None,
            controller: None,
            treasury: None,
            management_fee_rate: None,
            performance_fee_rate: None,
            instant_withdraw_penalty: None,
            penalty_duration: None,
            estimate_cycle_profit: None,
        }
    }
}

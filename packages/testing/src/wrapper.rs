use crate::setup::TestEnv;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use osmosis_test_tube::{OsmosisTestApp, RunnerExecuteResult, RunnerResult, SigningAccount, Wasm};

use interface::strategy::{ConfigResponse, ExecuteMsg, QueryMsg};

#[cw_serde]
pub enum AstroExecuteMsg {
    AppendPrice { price: Decimal },
}

// Execute Functions
impl TestEnv {
    pub fn repay_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        tokens_to_repay: Vec<Coin>,
        cycle_profit: Option<Decimal>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Repay {
            tokens_to_repay,
            cycle_profit,
        };

        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn withdraw_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        tokens_to_withdraw: Vec<Coin>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Withdraw { tokens_to_withdraw };

        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn set_vault_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        vault: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::SetVault {
            vault: vault.to_string(),
        };

        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn set_grants_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        grants: Vec<String>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::SetGrants { grants };

        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn update_config_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        grants: Option<Vec<String>>,
        controller: Option<String>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::UpdateConfig { grants, controller };

        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn set_astro_price_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        price: Decimal,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = AstroExecuteMsg::AppendPrice { price };

        wasm.execute(contract_addr, &msg, &[], signer)
    }
}

// Query Functions
impl TestEnv {
    pub fn query_config_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<ConfigResponse> {
        let query_msg = QueryMsg::Config {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_grants_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Vec<String>> {
        let query_msg = QueryMsg::Grants {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_spot_price_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Decimal> {
        let query_msg = QueryMsg::SpotPrice {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_twap_price_strategy(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        duration: u64,
    ) -> RunnerResult<Decimal> {
        let query_msg = QueryMsg::TwapPrice { duration };

        wasm.query(contract_addr, &query_msg)
    }
}

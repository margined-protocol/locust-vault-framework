use crate::setup::TestEnv;

use cosmwasm_std::Coin;
use interface::redemption::{
    ConfigResponse, ExecuteMsg, FundInfo, OwnerProposal, PendingRedemption, QueryMsg,
};
use neutron_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
use neutron_test_tube::{
    NeutronTestApp as OsmosisTestApp, RunnerExecuteResult, RunnerResult, SigningAccount, Wasm,
};

// Execute Functions
impl TestEnv {
    pub fn send_redemption(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        redemption: PendingRedemption,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::SendRedemption { redemption };
        wasm.execute(contract_addr, &msg, funds, signer)
    }

    pub fn claim_redemption(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        limit: Option<u32>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::ClaimRedemption { limit };
        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn update_config(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        add_fund: Option<FundInfo>,
        remove_fund: Option<FundInfo>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::UpdateConfig {
            add_fund,
            remove_fund,
        };
        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn propose_new_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        new_owner: String,
        duration: u64,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::ProposeNewOwner {
            new_owner,
            duration,
        };
        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn reject_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::RejectOwner {};
        wasm.execute(contract_addr, &msg, &[], signer)
    }

    pub fn claim_ownership(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::ClaimOwnership {};
        wasm.execute(contract_addr, &msg, &[], signer)
    }
}

// Query Functions
impl TestEnv {
    pub fn query_config_redemption(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<ConfigResponse> {
        let query_msg = QueryMsg::Config {};
        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_all_redemptions(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        start_after: Option<(String, u64)>,
        limit: Option<u32>,
    ) -> RunnerResult<Vec<PendingRedemption>> {
        let query_msg = QueryMsg::AllRedemptions { start_after, limit };
        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_redemptions(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        user: String,
        limit: Option<u32>,
    ) -> RunnerResult<Vec<PendingRedemption>> {
        let query_msg = QueryMsg::Redemptions { user, limit };
        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_owner(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<String> {
        let query_msg = QueryMsg::Owner {};
        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_ownership_proposal(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<OwnerProposal> {
        let query_msg = QueryMsg::GetOwnershipProposal {};
        wasm.query(contract_addr, &query_msg)
    }
}

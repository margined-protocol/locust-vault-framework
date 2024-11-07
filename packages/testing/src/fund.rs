use crate::setup::TestEnv;

use cosmwasm_std::{Addr, Coin as StdCoin, Decimal, Uint128};
use cw_vault_standard::VaultInfoResponse;
use interface::fund::{
    ConfigResponse, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, QueryMsg, StateResponse,
    UpdateConfig, VaultenatorExtensionExecuteMsg, VaultenatorExtensionQueryMsg, VersionResponse,
};
use neutron_std::types::{
    cosmos::base::v1beta1::Coin, cosmwasm::wasm::v1::MsgExecuteContractResponse,
};
use neutron_test_tube::{
    NeutronTestApp as OsmosisTestApp, RunnerExecuteResult, RunnerResult, SigningAccount, Wasm,
};
// use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse;
// use osmosis_test_tube::{OsmosisTestApp, RunnerExecuteResult, RunnerResult, SigningAccount, Wasm};
use vaultenator::ownership::OwnerProposal;

// Execute Functions
impl TestEnv {
    pub fn propose_new_owner_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        new_owner: String,
        duration: u64,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let propose_new_owner_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::ProposeNewOwner {
                new_owner,
                duration,
            },
        ));

        wasm.execute(contract_addr, &propose_new_owner_msg, &[], signer)
    }

    pub fn deposit_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Deposit {
            amount: Uint128::one(),
            recipient: None,
        };
        wasm.execute(contract_addr, &msg, funds, signer)
    }

    pub fn redeem_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Coin,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let msg = ExecuteMsg::Redeem {
            amount: Uint128::one(),
            recipient: None,
        };
        wasm.execute(contract_addr, &msg, &[amount], signer)
    }

    pub fn withdraw_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        tokens_to_withdraw: Vec<StdCoin>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_open_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::Withdraw { tokens_to_withdraw },
        ));
        wasm.execute(contract_addr, &set_open_msg, &[], signer)
    }

    pub fn repay_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        cycle_profit: Option<Decimal>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_open_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::Repay { cycle_profit },
        ));
        wasm.execute(contract_addr, &set_open_msg, funds, signer)
    }

    pub fn set_open_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_open_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::SetOpen {},
        ));
        wasm.execute(contract_addr, &set_open_msg, &[], signer)
    }

    pub fn set_pause_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_pause_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::Pause {},
        ));
        wasm.execute(contract_addr, &set_pause_msg, &[], signer)
    }

    pub fn update_config_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
        new_config: UpdateConfig,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let new_config_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::UpdateConfig { new_config },
        ));
        wasm.execute(contract_addr, &new_config_msg, &[], signer)
    }

    pub fn set_unpause_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let set_unpause_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::Unpause {},
        ));
        wasm.execute(contract_addr, &set_unpause_msg, &[], signer)
    }

    pub fn claim_ownership_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let claim_ownership_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::ClaimOwnership {},
        ));
        wasm.execute(contract_addr, &claim_ownership_msg, &[], signer)
    }

    pub fn reject_owner_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let reject_owner_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Vaultenator(
            VaultenatorExtensionExecuteMsg::RejectOwner {},
        ));
        wasm.execute(contract_addr, &reject_owner_msg, &[], signer)
    }
}

// Query Functions
impl TestEnv {
    pub fn query_owner_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Addr> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::Owner {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_ownership_proposal_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<OwnerProposal> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::OwnershipProposal {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_config_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<ConfigResponse> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::Config {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_state_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<StateResponse> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::State {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_info_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<VaultInfoResponse> {
        let query_msg = QueryMsg::Info {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_version_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<VersionResponse> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::Version {},
        ));

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_preview_deposit_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Uint128,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::PreviewDeposit { amount };

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_preview_redeem_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Uint128,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::PreviewRedeem { amount };

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_total_assets_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::TotalAssets {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_total_vault_token_supply_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::TotalVaultTokenSupply {};

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_convert_to_shares_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Uint128,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::ConvertToShares { amount };

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_convert_to_assets_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Uint128,
    ) -> RunnerResult<Uint128> {
        let query_msg = QueryMsg::ConvertToAssets { amount };

        wasm.query(contract_addr, &query_msg)
    }

    pub fn query_estimate_vault_assets_fund(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_addr: &str,
        amount: Uint128,
    ) -> RunnerResult<Vec<Coin>> {
        let query_msg = QueryMsg::VaultExtension(ExtensionQueryMsg::Vaultenator(
            VaultenatorExtensionQueryMsg::EstimateVaultAssets { amount },
        ));
        wasm.query(contract_addr, &query_msg)
    }
}

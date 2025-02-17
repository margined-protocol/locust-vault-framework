use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, Timestamp, Uint128};
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,               // manages contract configuration
    pub controller: String,          // manages contract balance sheet
    pub treasury: String,            // account fees are paid to
    pub redemption_contract: String, // address of the redemption contract
    pub strategy_cap: Uint128,       // maximum value of strategy deposits
    pub float: Option<Decimal>,      // percentage of balance sheet that can be withdrawn
    pub token0: String,
    pub token1: Option<String>,
    pub management_fee_rate: Decimal,
    pub performance_fee_rate: Decimal,
    pub vault_type: String,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    Vaultenator(VaultenatorExtensionExecuteMsg),
}

#[cw_serde]
pub enum ExtensionQueryMsg {
    Vaultenator(VaultenatorExtensionQueryMsg),
}

#[cw_serde]
pub enum VaultenatorExtensionQueryMsg {
    Config {},
    Owner {},
    OwnershipProposal {},
    State {},
    Version {},
    EstimateVaultAssets { amount: Uint128 },
    PendingRedemptions { limit: Option<u64> },
    UserRedemption { user: String },
    WithdrawableAmount {},
}

#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum VaultenatorExtensionExecuteMsg {
    CancelRedemption {},
    CreateRedemption {
        amount: Uint128,
    },
    ClaimOwnership {},
    Crank {},
    Pause {},
    ProposeNewOwner {
        new_owner: String,
        duration: u64,
    },
    RejectOwner {},
    SetOpen {},
    RegisterSudo {},
    UpdateConfig {
        new_config: UpdateConfig,
    },
    Unpause {},
    Withdraw {
        tokens_to_withdraw: Vec<Coin>,
    },
    Repay {
        cycle_profit: Option<Decimal>,
    },
    RepayQueue {
        cycle_profit: Option<Decimal>,
        limit: Option<u64>,
    },
}

#[cw_serde]
pub enum SudoMsg {
    BlockBeforeSend {
        from: String,
        to: String,
        amount: Coin,
    },
}

pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub controller: String,
    pub treasury: String,
    pub redemption_contract: String,
    pub strategy_cap: Uint128,
    pub float: Option<Decimal>,
    pub strategy_denom: String,
    pub token0: String,
    pub token1: Option<String>,
    pub management_fee_rate: Decimal,
    pub performance_fee_rate: Decimal,
    pub estimate_cycle_profit: Option<Decimal>,
    pub vault_type: String,
}

#[cw_serde]
pub struct Redemption {
    pub user: String,
    pub total_deposits: Uint128,
    pub timestamp: u64,
}

#[cw_serde]
pub struct StateResponse {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
    pub last_claim: Timestamp,
    pub total_staked_tokens: Uint128,
    pub total_withdrawn_tokens: Vec<Coin>,
    pub pending_management_fees: Vec<Coin>,
}

#[cw_serde]
pub struct UpdateConfig {
    pub strategy_cap: Option<Uint128>,
    pub controller: Option<String>,
    pub treasury: Option<String>,
    pub redemption_contract: Option<String>,
    pub management_fee_rate: Option<Decimal>,
    pub performance_fee_rate: Option<Decimal>,
    pub instant_withdraw_penalty: Option<Decimal>,
    pub penalty_duration: Option<u64>,
    pub estimate_cycle_profit: Option<Decimal>,
    pub float: Option<Option<Decimal>>, // Nested Option for set/remove functionality
}

#[cw_serde]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
}

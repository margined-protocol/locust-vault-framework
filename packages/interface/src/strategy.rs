use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, SignedDecimal};
use neutron_std::types::cosmos::bank::v1beta1::SendAuthorization;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,      // manages contract configuration
    pub controller: String, // manages grant execution
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>, // grants given to controller
    pub send_authorization: Option<SendAuthorization>, // optionalsend authorization - only used during instantiation
    pub pool_info: PoolInfo,                           // oracle support
}

#[cw_serde]
pub struct MigrateMsg {
    pub send_authorization: Option<SendAuthorization>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw {
        tokens_to_withdraw: Vec<Coin>,
    },
    Repay {
        tokens_to_repay: Vec<Coin>,
        cycle_profit: Option<SignedDecimal>,
    },
    RepayQueue {
        tokens_to_repay: Vec<Coin>,
        cycle_profit: Option<SignedDecimal>,
        limit: Option<u64>,
    },
    SetVault {
        vault: String,
    },
    SetGrants {
        grants: Vec<String>,
    },
    SetSendAuthorization {},
    UpdateConfig {
        grants: Option<Vec<String>>,
        controller: Option<String>,
    },
    ProposeNewOwner {
        new_owner: String,
        duration: u64,
    },
    RejectOwner {},
    ClaimOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(Vec<String>)]
    Grants {},
    #[returns(Decimal)]
    SpotPrice {},
    #[returns(Decimal)]
    TwapPrice { duration: u64 },
    #[returns(String)]
    Owner {},
    #[returns(OwnerProposal)]
    GetOwnershipProposal {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub controller: String,
    pub vault: Option<String>,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub send_authorization: Option<SendAuthorization>,
    pub pool_info: PoolInfo,
    pub name: String,
    pub version: String,
}

#[cw_serde]
pub enum PoolInfo {
    Astroport {
        pool_address: String,
        token0: String,
        token1: String,
    },
    Drop {
        address: String,
        inverted: bool,
    },
    Osmosis {
        id: u64,
        token0: String,
        token1: String,
    },
    Slinky {
        base: String,
        quote: String,
        timeout: u64, // time in seconds before we consider the price stale
    },
}

#[cw_serde]
pub struct OwnerProposal {
    pub owner: Addr,
    pub expiry: u64,
}

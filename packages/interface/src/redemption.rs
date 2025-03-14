use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use std::fmt;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,                    // manages contract configuration
    pub whitelisted_funds: Vec<FundInfo>, // list whitelisted fund contracts
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendRedemption {
        redemption: PendingRedemption, // Redemptions must be sent individually
    },
    ClaimRedemption {
        limit: Option<u32>, // Pagination limit
    },
    UpdateConfig {
        add_fund: Option<FundInfo>,
        remove_fund: Option<FundInfo>,
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
    #[returns(Vec<PendingRedemption>)]
    AllRedemptions {
        start_after: Option<(String, u64)>,
        limit: Option<u32>,
    },
    #[returns(Vec<PendingRedemption>)]
    Redemptions { user: String, limit: Option<u32> },
    #[returns(String)]
    Owner {},
    #[returns(OwnerProposal)]
    GetOwnershipProposal {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub whitelisted_funds: Vec<FundInfo>,
    pub name: String,
    pub version: String,
}

#[cw_serde]
pub struct PendingRedemption {
    pub user: String,
    pub funds: Vec<Coin>,
    pub timestamp: u64,
    pub source: String,
}

#[cw_serde]
pub struct FundInfo {
    pub address: String,
    pub metadata: String,
}

impl fmt::Display for FundInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.address, self.metadata)
    }
}

#[cw_serde]
pub struct OwnerProposal {
    pub owner: Addr,
    pub expiry: u64,
}

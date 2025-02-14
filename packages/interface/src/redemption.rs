use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};

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
pub enum QueryMsg {
    Config {},
    AllRedemptions { user: String, limit: Option<u32> },
    Redemptions { user: String, limit: Option<u32> },
    Owner {},
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

#[cw_serde]
pub struct OwnerProposal {
    pub owner: Addr,
    pub expiry: u64,
}

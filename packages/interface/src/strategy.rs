use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,      // manages contract configuration
    pub controller: String, // manages grant execution
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>, // grants given to controller
    pub pool_info: PoolInfo, // oracle support
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Withdraw {
        tokens_to_withdraw: Vec<Coin>,
    },
    Repay {
        tokens_to_repay: Vec<Coin>,
        cycle_profit: Option<Decimal>,
    },
    SetVault {
        vault: String,
    },
    SetGrants {
        grants: Vec<String>,
    },
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
pub enum QueryMsg {
    Config {},
    Grants {},
    SpotPrice {},
    TwapPrice { duration: u64 },
    Owner {},
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

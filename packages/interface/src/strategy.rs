use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub controller: String,
    pub token0: String,
    pub token1: Option<String>,
    pub grants: Vec<String>,
    pub pool_info: PoolInfo,
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
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Grants {},
    SpotPrice {},
    TwapPrice { duration: u64 },
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
    Osmosis {
        id: u64,
        token0: String,
        token1: String,
    },
    Neutron {},
    Astroport {
        pool_address: String,
        token0: String,
        token1: String,
    },
}

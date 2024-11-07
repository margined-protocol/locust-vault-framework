use crate::state::{Config, CONFIG};

#[cfg(feature = "astroport")]
use cosmwasm_schema::cw_serde;
#[cfg(feature = "astroport")]
use cosmwasm_std::{to_json_binary, Addr, QueryRequest, Uint128, WasmQuery};
use cosmwasm_std::{Decimal, Deps, Env, StdResult};
#[cfg(feature = "slinky")]
use cosmwasm_std::{StdError, Uint128};
use cw2::get_contract_version;
use interface::strategy::{ConfigResponse, PoolInfo};
#[cfg(feature = "slinky")]
use neutron_std::types::slinky::{oracle::v1::OracleQuerier, types::v1::CurrencyPair};
#[cfg(feature = "osmosis")]
use osmosis_std::{
    shim::Timestamp as OsmosisTimestamp,
    types::osmosis::{poolmanager::v1beta1::PoolmanagerQuerier, twap::v1beta1::TwapQuerier},
};
#[cfg(any(feature = "osmosis", feature = "slinky"))]
use std::str::FromStr;

#[cfg(feature = "astroport")]
#[cw_serde]
pub struct OracleObservation {
    pub timestamp: u64,
    pub price: Decimal,
}

#[cfg(feature = "astroport")]
#[cw_serde]
pub struct SimulationResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

#[cfg(feature = "astroport")]
#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

#[cfg(feature = "astroport")]
#[cw_serde]
pub enum AssetInfo {
    Token { contract_addr: Addr },
    NativeToken { denom: String },
}

pub fn query_config(deps: &Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;

    let version = get_contract_version(deps.storage)?;

    Ok(ConfigResponse {
        admin: config.admin,
        controller: config.controller,
        vault: config.vault,
        token0: config.token0,
        token1: config.token1,
        grants: config.grants,
        pool_info: config.pool_info,
        name: version.contract,
        version: version.version,
    })
}

pub fn query_grants(deps: &Deps) -> StdResult<Vec<String>> {
    let config: Config = CONFIG.load(deps.storage)?;

    Ok(config.grants)
}

#[cfg(feature = "astroport")]
pub fn query_spot_price(deps: &Deps, env: Env) -> StdResult<Decimal> {
    let config: Config = CONFIG.load(deps.storage)?;

    #[cw_serde]
    pub enum QueryMsg {
        Simulation {
            offer_asset: Asset,
            ask_asset_info: Option<AssetInfo>,
        },
        Observe {
            seconds_ago: u64,
        },
    }

    let (pool_address, token0, token1) = match config.pool_info {
        PoolInfo::Osmosis { .. } => unimplemented!(),
        PoolInfo::Slinky { .. } => unimplemented!(),
        PoolInfo::Astroport {
            pool_address,
            token0,
            token1,
        } => (pool_address, token0, token1),
    };

    let amount_in = Uint128::from(1_000_000u128);

    let offer_asset = Asset {
        info: AssetInfo::NativeToken { denom: token0 },
        amount: amount_in,
    };

    let ask_asset_info = AssetInfo::NativeToken { denom: token1 };

    let res: SimulationResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address,
        msg: to_json_binary(&QueryMsg::Simulation {
            offer_asset,
            ask_asset_info: Some(ask_asset_info),
        })?,
    }))?;

    let price = Decimal::from_ratio(res.return_amount, amount_in);

    Ok(price)
}

#[cfg(feature = "astroport")]
pub fn query_twap_price(deps: &Deps, _: Env, duration: u64) -> StdResult<Decimal> {
    let config = CONFIG.load(deps.storage)?;

    #[cw_serde]
    pub enum QueryMsg {
        Simulation {
            offer_asset: Asset,
            ask_asset_info: Option<AssetInfo>,
        },
        Observe {
            seconds_ago: u64,
        },
    }

    let (pool_address, _, _) = match config.pool_info {
        PoolInfo::Osmosis { .. } => unimplemented!(),
        PoolInfo::Slinky { .. } => unimplemented!(),
        PoolInfo::Astroport {
            pool_address,
            token0,
            token1,
        } => (pool_address, token0, token1),
    };

    let res: OracleObservation = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address,
        msg: to_json_binary(&QueryMsg::Observe {
            seconds_ago: duration,
        })?,
    }))?;

    Ok(res.price)
}

#[cfg(feature = "osmosis")]
pub fn query_spot_price(deps: &Deps, env: Env) -> StdResult<Decimal> {
    let config: Config = CONFIG.load(deps.storage)?;

    let (id, token0, token1) = match config.pool_info {
        PoolInfo::Osmosis { id, token0, token1 } => (id, token0, token1),
        PoolInfo::Slinky { .. } => unimplemented!(),
        PoolInfo::Astroport { .. } => unimplemented!(),
    };

    let querier = PoolmanagerQuerier::new(&deps.querier);

    let res = querier.spot_price(id, token0, token1)?;

    let price = Decimal::from_str(&res.spot_price).unwrap();

    Ok(price)
}

#[cfg(feature = "osmosis")]
pub fn query_twap_price(deps: &Deps, env: Env, duration: u64) -> StdResult<Decimal> {
    let config = CONFIG.load(deps.storage)?;

    let (id, token0, token1) = match config.pool_info {
        PoolInfo::Osmosis { id, token0, token1 } => (id, token0, token1),
        PoolInfo::Slinky { .. } => unimplemented!(),
        PoolInfo::Astroport { .. } => unimplemented!(),
    };

    let querier = TwapQuerier::new(&deps.querier);

    let start_time = env.block.time.minus_seconds(duration);

    let start_time = OsmosisTimestamp {
        seconds: start_time.seconds() as i64,
        nanos: start_time.subsec_nanos() as i32,
    };

    let res = querier.arithmetic_twap_to_now(id, token0, token1, Some(start_time))?;

    let price = Decimal::from_str(&res.arithmetic_twap).unwrap();

    Ok(price)
}

#[cfg(feature = "slinky")]
pub fn query_spot_price(deps: &Deps, env: Env) -> StdResult<Decimal> {
    get_slinky_price(deps, env)
}

#[cfg(feature = "slinky")]
pub fn query_twap_price(deps: &Deps, env: Env, _duration: u64) -> StdResult<Decimal> {
    get_slinky_price(deps, env)
}

#[cfg(feature = "slinky")]
fn get_slinky_price(deps: &Deps, env: Env) -> StdResult<Decimal> {
    deps.api.debug("get_slinky_price");
    let config: Config = CONFIG.load(deps.storage)?;

    let (base, quote, timeout) = match config.pool_info {
        PoolInfo::Osmosis { .. } => unimplemented!(),
        PoolInfo::Slinky {
            base,
            quote,
            timeout,
        } => (base, quote, timeout),
        PoolInfo::Astroport { .. } => unimplemented!(),
    };

    let querier = OracleQuerier::new(&deps.querier);

    let res = querier.get_price(Some(CurrencyPair { base, quote }))?;

    let price = match res.price {
        None => {
            return Err(StdError::generic_err("Price not available"));
        }
        Some(p) => {
            let timestamp = p.block_timestamp.unwrap();

            deps.api.debug(&format!("Price: {:?}", p));

            if timestamp.seconds < (env.block.time.seconds() - timeout) as i64 {
                return Err(StdError::generic_err("Price is stale"));
            }

            p.price
        }
    };

    let uint_price = Uint128::from_str(&price)?;

    let price = Decimal::from_atomics(uint_price, res.decimals as u32).unwrap();

    Ok(price)
}

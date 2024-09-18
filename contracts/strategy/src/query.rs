use crate::state::{Config, CONFIG};

use cosmwasm_std::{Decimal, Deps, Env, StdResult};
use cw2::get_contract_version;
use interface::strategy::{ConfigResponse, PoolInfo};
use osmosis_std::{
    shim::Timestamp as OsmosisTimestamp,
    types::osmosis::{poolmanager::v1beta1::PoolmanagerQuerier, twap::v1beta1::TwapQuerier},
};
use std::str::FromStr;

pub fn query_config(deps: &Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;

    let version = get_contract_version(deps.storage)?;

    Ok(ConfigResponse {
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

pub fn query_spot_price(deps: &Deps) -> StdResult<Decimal> {
    let config: Config = CONFIG.load(deps.storage)?;

    let (id, token0, token1) = match config.pool_info {
        PoolInfo::Osmosis { id, token0, token1 } => (id, token0, token1),
        PoolInfo::Neutron {} => unimplemented!(),
    };

    let querier = PoolmanagerQuerier::new(&deps.querier);

    let res = querier.spot_price(id, token0, token1)?;

    let price = Decimal::from_str(&res.spot_price).unwrap();

    Ok(price)
}

pub fn query_twap_price(deps: &Deps, env: Env, duration: u64) -> StdResult<Decimal> {
    let config = CONFIG.load(deps.storage)?;

    let (id, token0, token1) = match config.pool_info {
        PoolInfo::Osmosis { id, token0, token1 } => (id, token0, token1),
        PoolInfo::Neutron {} => unimplemented!(),
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

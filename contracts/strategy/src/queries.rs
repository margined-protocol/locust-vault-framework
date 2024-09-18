use common::utils::calculate_total_value;
use cosmwasm_std::{Coin, Deps, Env, QuerierWrapper, SignedDecimal, StdError, StdResult, Uint128};
use osmosis_std::types::{
    cosmos::bank::v1beta1::BankQuerier,
    osmosis::concentratedliquidity::v1beta1::{
        ConcentratedliquidityQuerier, FullPositionBreakdown, Pool, UserPositionsResponse,
    },
    osmosis::{poolmanager::v1beta1::PoolmanagerQuerier, twap::v1beta1::TwapQuerier},
};
use std::str::FromStr;
use vaultenator::errors::ContractError;

use crate::{config::Config, helpers::TWAP_PERIOD};

pub fn query_spot_price(
    querier: &QuerierWrapper,
    pool_id: &u64,
    base_denom: &str,
    quote_denom: &str,
) -> Result<SignedDecimal, ContractError> {
    let poolmanager = PoolmanagerQuerier::new(querier);

    let res = poolmanager.spot_price(*pool_id, base_denom.to_string(), quote_denom.to_string())?;
    SignedDecimal::from_str(&res.spot_price).map_err(|_| {
        ContractError::Std(StdError::GenericErr {
            msg: "Failed to parse spot price".to_string(),
        })
    })
}

pub fn query_twap_price(
    querier: &QuerierWrapper,
    pool_id: &u64,
    base_denom: &str,
    quote_denom: &str,
) -> Result<SignedDecimal, ContractError> {
    let twap = TwapQuerier::new(querier);

    let res =
        twap.arithmetic_twap_to_now(*pool_id, base_denom.to_string(), quote_denom.to_string())?;
    SignedDecimal::from_str(&res.arithmetic_twap).map_err(|_| {
        ContractError::Std(StdError::GenericErr {
            msg: "Failed to parse spot price".to_string(),
        })
    })
}

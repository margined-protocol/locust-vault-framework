use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Decimal, Deps, QuerierWrapper, QueryRequest, StdError, StdResult, Uint128,
    WasmQuery,
};
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use std::str::FromStr;
use vaultenator::errors::ContractError;

#[cw_serde]
pub enum StrategyQueryMsg {
    SpotPrice {},
    TwapPrice { duration: u64 },
}

pub fn get_balance(deps: &Deps, address: &str, denom: &str) -> StdResult<Uint128> {
    let bank = BankQuerier::new(&deps.querier);

    let res = bank.balance(address.to_string(), denom.to_string())?;

    let amount = match res.balance {
        Some(amount) => Uint128::from_str(&amount.amount)?,
        None => return Err(StdError::generic_err("No balance found")),
    };

    Ok(amount)
}

pub fn get_total_supply(deps: &Deps, denom: &str) -> StdResult<Uint128> {
    let bank = BankQuerier::new(&deps.querier);

    let res = bank.supply_of(denom.to_string())?;

    let amount = match res.amount {
        Some(amount) => Uint128::from_str(&amount.amount)?,
        None => return Err(StdError::generic_err("No supply found")),
    };

    Ok(amount)
}

pub fn query_spot_price(
    querier: &QuerierWrapper,
    contract_addr: &str,
) -> Result<Decimal, ContractError> {
    let res: Decimal = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_json_binary(&StrategyQueryMsg::SpotPrice {})?,
    }))?;

    Ok(res)
}

pub fn query_twap_price(
    querier: &QuerierWrapper,
    contract_addr: &str,
    duration: u64,
) -> Result<Decimal, ContractError> {
    let res: Decimal = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_json_binary(&StrategyQueryMsg::TwapPrice { duration })?,
    }))?;

    Ok(res)
}

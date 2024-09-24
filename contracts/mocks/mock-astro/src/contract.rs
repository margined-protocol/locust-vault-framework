use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128,
};
use cw_storage_plus::Item;

pub const KEY_PRICES: Item<Decimal> = Item::new("prices");
pub const ONE_SIX_DECIMALS: Uint128 = Uint128::new(1_000_000u128);

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AppendPrice { price: Decimal },
}

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

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct OracleObservation {
    pub timestamp: u64,
    pub price: Decimal,
}

#[cw_serde]
pub struct SimulationResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
}

#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

#[cw_serde]
pub enum AssetInfo {
    Token { contract_addr: Addr },
    NativeToken { denom: String },
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::AppendPrice { price } => append_price(deps, info, price),
    }
}

/// this is a mock function that enables storage of data
/// by the contract owner will be replaced by integration
/// with on-chain price oracles in the future.
#[cfg(not(tarpaulin_include))]
pub fn append_price(deps: DepsMut, _info: MessageInfo, price: Decimal) -> StdResult<Response> {
    KEY_PRICES.save(deps.storage, &price)?;

    Ok(Response::default())
}

#[cfg(not(tarpaulin_include))]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Simulation { .. } => to_json_binary(&return_simulation(deps)?),
        QueryMsg::Observe { .. } => to_json_binary(&return_observation(deps, env)?),
    }
}

/// this is a mock query that returns a dummy simulation response
#[cfg(not(tarpaulin_include))]
pub fn return_simulation(deps: Deps) -> StdResult<SimulationResponse> {
    let price = KEY_PRICES.load(deps.storage)?;

    let return_amount = price * ONE_SIX_DECIMALS;

    Ok(SimulationResponse {
        return_amount,
        spread_amount: Uint128::zero(),
        commission_amount: Uint128::zero(),
    })
}

/// this is a mock query that returns a dummy observation response
#[cfg(not(tarpaulin_include))]
pub fn return_observation(deps: Deps, env: Env) -> StdResult<OracleObservation> {
    // give a timestamp 30 seconds ago
    let timestamp = env.block.time.seconds() - 30u64;

    Ok(OracleObservation {
        timestamp,
        price: KEY_PRICES.load(deps.storage)?,
    })
}

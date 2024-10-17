use crate::{
    config::Config,
    queries::{get_balance, get_total_supply, query_twap_price},
    state::{State, TWAP_PERIOD},
};

use cosmwasm_std::{
    coin, ensure, Coin, Decimal, Deps, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw_margined::utils::may_pay_two_denoms;
use cw_utils::must_pay;
use std::{fmt::Display, str::FromStr};
use vaultenator::errors::ContractError;

pub fn calculate_vault_assets(
    deps: &Deps,
    config: &Config,
    state: &State,
    contract_addr: &str,
) -> StdResult<Uint128> {
    let tokens = get_vault_coins(deps, config, contract_addr)?;

    let mut assets = Vec::new();
    for token in tokens {
        let total_withdrawn = state
            .total_withdrawn_tokens
            .get(&token.denom)
            .cloned()
            .unwrap_or(Uint128::zero());

        let current_assets = token.amount.checked_add(total_withdrawn)?;

        assets.push(Coin {
            denom: token.denom,
            amount: current_assets,
        });
    }

    let total_token = get_deposit_value(deps, config, assets)?;

    Ok(total_token)
}

pub fn calculate_amount_to_mint(
    current_assets: &Uint128,
    previous_assets: &Uint128,
    total_supply: Uint128,
) -> Uint128 {
    let delta_liquidity = current_assets.saturating_sub(*previous_assets);

    let normalized_delta = Decimal::from_ratio(delta_liquidity, *previous_assets);

    normalized_delta * total_supply
}

pub fn get_strategy_denom(env: &Env, contract_name: &str) -> String {
    format!("factory/{}/{}", env.contract.address, contract_name).to_string()
}

// Helper function to map any Display error to ContractError
pub fn map_to_contract_error<E: Display>(e: E) -> ContractError {
    ContractError::Std(StdError::generic_err(e.to_string()))
}

// pub fn calculate_total_value(
//     deps: &Deps,
//     env: &Env,
//     pool_id: &u64,
//     quote_denom: &str,
//     assets: &[&BaseCoin],
//     twap_period: &u64,
// ) -> StdResult<Uint128> {
//     let mut total = Uint128::zero();
//     let start_time = env.block.time.minus_seconds(*twap_period);

//     for asset in assets {
//         if asset.denom == quote_denom {
//             let amount = Uint128::from_str(&asset.amount.to_string())?;

//             total = total.checked_add(amount)?;
//         } else {
//             let twap =
//                 get_arithmetic_twap_to_now(deps, pool_id, &asset.denom, quote_denom, start_time)?;

//             let amount = Uint128::from_str(&asset.amount.to_string())?;

//             total += twap * amount;
//         }
//     }
//     Ok(total)
// }

pub fn calculate_total_value(
    deps: &Deps,
    controller_addr: &str,
    quote_denom: &str,
    assets: &[&Coin],
) -> StdResult<Uint128> {
    let mut total = Uint128::zero();
    for asset in assets {
        if asset.denom == quote_denom {
            let amount = Uint128::from_str(&asset.amount.to_string())?;

            total = total.checked_add(amount)?;
        } else {
            let twap = query_twap_price(&deps.querier, controller_addr, TWAP_PERIOD)
                .map_err(|_| StdError::generic_err("Failed to fetch TWAP price"))?;

            let amount = Uint128::from_str(&asset.amount.to_string())?;

            total += twap * amount;
        }
    }
    Ok(total)
}

pub fn calculate_performance_fees(coins: Vec<Coin>, fee_rate: Decimal) -> StdResult<Vec<Coin>> {
    let mut fees = Vec::new();

    for coin in coins {
        let initial_amount = coin.amount;
        let fee = initial_amount * fee_rate;

        if !fee.is_zero() {
            fees.push(cosmwasm_std::coin(fee.u128(), coin.denom.clone()));
        }
    }

    Ok(fees)
}

pub fn check_strategy_cap(config: &Config, state: &State) -> Result<Response, ContractError> {
    if state.total_staked_tokens > config.strategy_cap {
        return Err(ContractError::StrategyCapExceeded {});
    }

    Ok(Response::default())
}

pub fn check_is_valid_token(config: &Config, denom: &str) -> StdResult<()> {
    let is_valid = if let Some(token1_denom) = &config.token1 {
        // Check if the denomination matches either the alternative or deposit denomination
        denom == token1_denom || denom == config.token0
    } else {
        // Check if the denomination matches the deposit denomination only
        denom == config.token0
    };

    // Use the result of the check to decide whether to return Ok or an error
    ensure!(is_valid, StdError::generic_err("Invalid token"));

    Ok(())
}

pub fn coins_to_string(coins: Vec<Coin>) -> String {
    coins
        .iter()
        .map(|coin| coin.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn get_vault_coins(deps: &Deps, config: &Config, contract_addr: &str) -> StdResult<Vec<Coin>> {
    let mut tokens = Vec::new();

    tokens.push(coin(
        get_balance(deps, contract_addr, &config.token0)?.into(),
        &config.token0,
    ));

    if let Some(denom) = &config.token1 {
        tokens.push(coin(get_balance(deps, contract_addr, denom)?.into(), denom));
    }

    Ok(tokens)
}

pub fn get_amount_to_mint(
    deps: &Deps,
    current_assets: &Uint128,
    previous_assets: &Uint128,
    strategy_denom: &str,
) -> StdResult<Uint128> {
    let total_supply = get_total_supply(deps, strategy_denom)?;

    let amount_to_mint = calculate_amount_to_mint(current_assets, previous_assets, total_supply);

    Ok(amount_to_mint)
}

pub fn get_deposit_value(deps: &Deps, config: &Config, tokens: Vec<Coin>) -> StdResult<Uint128> {
    let deposit_value = match &config.token1 {
        Some(_) => {
            let base_tokens: Vec<Coin> = tokens
                .iter()
                .map(|t| Coin {
                    denom: t.denom.clone(), // Assuming 'denom' is a String or has `clone()`
                    amount: t.amount,       // Assuming 'amount' is Uint128 or similar
                })
                .collect();

            let base_tokens: Vec<&Coin> = base_tokens.iter().collect();

            calculate_total_value(
                deps,
                config.controller.as_str(),
                &config.token0,
                base_tokens.as_slice(),
            )?
        }
        None => tokens.first().unwrap().amount,
    };

    Ok(deposit_value)
}

pub fn get_sent_tokens(info: &MessageInfo, config: &Config) -> StdResult<Vec<Coin>> {
    let mut tokens = Vec::new();

    match &config.token1 {
        Some(token1_denom) => {
            let (token0_deposit, token1_deposit) =
                may_pay_two_denoms(info, &config.token0, token1_denom).map_err(|_| {
                    StdError::generic_err("Failed to retrieve token deposits for two denoms")
                })?;

            let token0_coin = coin(token0_deposit.u128(), &config.token0);
            let token1_coin = coin(token1_deposit.u128(), token1_denom);

            if !token0_coin.amount.is_zero() {
                tokens.push(token0_coin);
            }

            if !token1_coin.amount.is_zero() {
                tokens.push(token1_coin);
            }
        }
        None => {
            let token_deposit = must_pay(info, &config.token0)
                .map_err(|_| StdError::generic_err("Failed to retrieve token deposit"))?;

            let token_coin = coin(token_deposit.u128(), &config.token0);

            tokens.push(token_coin);
        }
    };

    Ok(tokens)
}

pub fn get_token_deposits(config: &Config, tokens: Vec<Coin>) -> StdResult<(Coin, Option<Coin>)> {
    // Find token0 based on config.token0
    let token0 = tokens
        .iter()
        .find(|coin| coin.denom == config.token0)
        .cloned() // Clone to get ownership of the Coin
        .ok_or_else(|| StdError::generic_err("no deposit denom found"))?; // Handle the case where no token matches

    // Find token1 based on config.token1 if it exists
    let token1 = if let Some(token1_denom) = &config.token1 {
        tokens
            .iter()
            .find(|coin| coin.denom == *token1_denom)
            .cloned() // Clone to get ownership of the Coin
    } else {
        None
    };

    Ok((token0, token1))
}

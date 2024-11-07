use crate::{
    config::Config,
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{event_fees, event_mint, event_redeem},
    helpers::{get_assets, get_management_fees, get_token_deposits, get_vault_coins},
    messages::{create_bank_message, create_burn_message, create_mint_message},
    state::State,
};

use cosmwasm_std::{
    coin, Addr, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use vaultenator::{config::Configure, errors::ContractError, state::ManageState};

pub fn process_management_fees_and_modify_response(
    mut deps: DepsMut,
    mut response: Response,
    env: Env,
    deposits: Option<Vec<Coin>>,
) -> Result<(Response, DepsMut), ContractError> {
    let config = Config::get_from_storage(deps.as_ref())?;
    let mut state = State::get_from_storage(deps.as_ref())?;

    let mut total_assets = get_assets(
        &deps.as_ref(),
        &config,
        &state,
        env.contract.address.as_str(),
    )?;
    let balances = get_vault_coins(&deps.as_ref(), &config, env.contract.address.as_str())?;

    if deposits.is_some() {
        let deposits = deposits.unwrap();
        for deposit in deposits {
            if let Some(token) = total_assets.iter_mut().find(|t| t.denom == deposit.denom) {
                // Subtract deposit amount from token amount
                token.amount = token.amount.saturating_sub(deposit.amount);
            }
        }
    }

    let (management_fees, pending_fees) = get_management_fees(
        env.block.time.seconds(),
        state.last_claim.seconds(),
        total_assets,
        balances.clone(),
        state.pending_management_fees.clone(),
        config.management_fee_rate,
    )?;

    state.update_last_claim(env.block.time)?;
    state.update_pending_management_fees(pending_fees)?;
    state.save_to_storage(&mut deps)?;

    response = response
        .add_event(event_fees(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            "management",
            management_fees.clone(),
        ))
        .add_message(create_bank_message(
            config.treasury.clone(),
            management_fees.clone(),
        ));

    Ok((response, deps))
}

// Helper to mint tokens and add the mint message to the response if necessary
pub fn process_deposit(
    response: Response,
    amount_to_mint: Uint128,
    sender: &Addr,
    contract_address: &Addr,
    config: &Config,
) -> Result<Response, ContractError> {
    if amount_to_mint.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "No tokens to mint - increase deposit amount",
        )));
    }

    let mint_msg = create_mint_message(
        contract_address,
        sender.to_string(),
        amount_to_mint,
        config.strategy_denom.to_string(),
    );

    Ok(response
        .add_event(event_mint(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            sender.as_ref(),
            &amount_to_mint.to_string(),
        ))
        .add_message(mint_msg))
}

// Helper to build response messages
pub fn process_redeem(
    mut response: Response,
    info: &MessageInfo,
    assets_to_redeem: Vec<Coin>,
    config: &Config,
    env: &Env,
    strategy_denom_sent: Uint128,
) -> StdResult<Response> {
    for asset in assets_to_redeem.iter().filter(|a| !a.amount.is_zero()) {
        response = response.add_message(create_bank_message(
            info.sender.to_string(),
            vec![asset.clone()],
        ));
    }

    let (token0, token1) = get_token_deposits(config, assets_to_redeem.clone())?;

    let burn_msg = create_burn_message(
        &env.contract.address,
        env.contract.address.to_string(),
        strategy_denom_sent,
        config.strategy_denom.clone(),
    );

    Ok(response.add_message(burn_msg).add_event(event_redeem(
        CONTRACT_VERSION,
        CONTRACT_NAME,
        info.sender.as_ref(),
        token0,
        token1,
        coin(0u128, config.token0.clone()),
        None,
    )))
}

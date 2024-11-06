use crate::{
    config::Config,
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    events::event_fees,
    helpers::{get_assets, get_management_fees, get_vault_coins},
    messages::create_bank_message,
    state::State,
};

use cosmwasm_std::{Coin, DepsMut, Env, Response};
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

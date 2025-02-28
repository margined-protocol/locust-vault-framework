use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{event_burn, event_fees, event_mint, event_redeem, event_repay},
    helpers::{
        calculate_performance_fees, get_management_fees, get_sent_tokens, get_token_deposits,
        get_total_vault_assets, get_vault_coins,
    },
    messages::{
        create_bank_message, create_burn_message, create_mint_message, create_redemption_message,
    },
    storage::{config::Config, state::State},
};

use cosmwasm_std::{
    coin, Addr, Coin, Decimal, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
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

    let mut total_assets = get_total_vault_assets(
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

pub fn process_redeem(
    mut response: Response,
    recipient: &Addr,
    assets_to_redeem: Vec<Coin>,
    config: &Config,
    env: &Env,
    strategy_denom_sent: Uint128,
) -> StdResult<Response> {
    response = response.add_message(create_redemption_message(
        config.redemption_contract.to_string(),
        recipient.to_string(),
        assets_to_redeem.clone(),
        env.block.time.seconds(),
        env.contract.address.to_string(),
    ));

    let (token0, token1) = get_token_deposits(config, assets_to_redeem.clone())?;

    let burn_msg = create_burn_message(
        &env.contract.address,
        env.contract.address.to_string(),
        strategy_denom_sent,
        config.strategy_denom.clone(),
    );

    Ok(response.add_message(burn_msg).add_events([
        event_burn(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            recipient.as_ref(),
            &strategy_denom_sent.to_string(),
            None,
        ),
        event_redeem(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            recipient.as_ref(),
            token0,
            token1,
            coin(0u128, config.token0.clone()),
            None,
        ),
    ]))
}

pub fn process_repayments(
    info: &MessageInfo,
    config: &Config,
    state: &mut State,
    deps: &mut DepsMut,
    cycle_profit: Option<Decimal>,
    mut response: Response,
) -> Result<Response, ContractError> {
    let repayment_tokens = get_sent_tokens(info, config)?;

    let profit_percentage =
        cycle_profit.unwrap_or_else(|| config.estimate_cycle_profit.unwrap_or(Decimal::zero()));

    for repayment in repayment_tokens.iter() {
        let total_withdrawn = state.get_total_withdrawn_tokens(&repayment.denom);

        let profit = if repayment.amount < total_withdrawn {
            repayment.amount.mul_floor(profit_percentage)
        } else {
            repayment.amount.saturating_sub(total_withdrawn)
        };

        let amount_repaid = repayment.amount.saturating_sub(profit);

        // Update state with repayment and save to storage
        state.remove_from_total_withdrawn_tokens(amount_repaid, &repayment.denom)?;
        state.save_to_storage(deps)?;

        // Calculate performance fees
        let performance_fee = calculate_performance_fees(
            vec![Coin {
                denom: repayment.denom.clone(),
                amount: profit,
            }],
            config.performance_fee_rate,
        )?;

        // Create performance fee event
        let performance_fee_event = event_fees(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            "performance",
            performance_fee.clone(),
        );

        // Add events and bank message to response
        response = response
            .add_events([
                event_repay(
                    config.controller.as_str(),
                    Coin {
                        denom: repayment.denom.clone(),
                        amount: repayment.amount,
                    },
                ),
                performance_fee_event,
            ])
            .add_message(create_bank_message(
                config.treasury.clone(),
                performance_fee.clone(),
            ));
    }

    Ok(response)
}

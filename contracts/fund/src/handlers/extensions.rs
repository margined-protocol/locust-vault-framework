use crate::{
    events::{event_cancel_redemption, event_create_redemption, event_withdraw},
    handlers::helpers::{calculate_share_to_burn, calculate_total_assets_redeemable},
    helpers::{
        calculate_amount_withdrawable, check_is_valid_token, ensure_no_duplicate_denoms,
        get_vault_balance, map_to_contract_error,
    },
    messages::create_bank_message,
    process::{process_management_fees_and_modify_response, process_redeem, process_repayments},
    storage::{
        config::Config,
        queue::{add_to_queue, redemptions, remove_from_queue},
        state::{update_user_deposit, State},
    },
};

use cosmwasm_std::{
    coin, ensure, Coin, Decimal, DepsMut, Env, MessageInfo, Order, Response, Uint128,
};
use cw_utils::{must_pay, nonpayable};
use vaultenator::{config::Configure, errors::ContractError, state::ManageState};

pub fn handle_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    tokens_to_withdraw: Vec<Coin>,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(map_to_contract_error)?;
    let (mut response, mut deps) =
        process_management_fees_and_modify_response(deps, Response::default(), env.clone(), None)?;

    State::is_open_and_unpaused(deps.as_ref())?;

    let config = Config::get_from_storage(deps.as_ref())?;
    let mut state = State::get_from_storage(deps.as_ref())?;

    ensure!(
        config.controller == info.sender.to_string(),
        ContractError::Unauthorized {}
    );

    ensure_no_duplicate_denoms(&tokens_to_withdraw)?;

    for token in tokens_to_withdraw.iter() {
        check_is_valid_token(&config, &token.denom)?;

        let max_withdrawable = calculate_amount_withdrawable(
            &deps.as_ref(),
            &config,
            &state,
            env.contract.address.as_str(),
            &token.denom,
        )?;

        let amount_to_withdraw = max_withdrawable.min(token.amount);

        // Create withdrawal message
        if !amount_to_withdraw.is_zero() {
            // Update total staked assets
            state.add_to_total_withdrawn_tokens(amount_to_withdraw, &token.denom)?;
            state.save_to_storage(&mut deps)?;

            let withdraw_amount = Coin {
                denom: token.denom.to_string(),
                amount: amount_to_withdraw,
            };

            let msg = create_bank_message(config.controller.clone(), vec![withdraw_amount.clone()]);

            response = response
                .add_event(event_withdraw(info.sender.as_str(), withdraw_amount))
                .add_message(msg);
        }
    }

    Ok(response)
}

pub fn handle_repay(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cycle_profit: Option<Decimal>,
) -> Result<Response, ContractError> {
    let (response, mut deps) =
        process_management_fees_and_modify_response(deps, Response::default(), env.clone(), None)?;

    State::is_open_and_unpaused(deps.as_ref())?;

    let config = Config::get_from_storage(deps.as_ref())?;
    let mut state = State::get_from_storage(deps.as_ref())?;

    ensure!(
        config.controller == info.sender.to_string(),
        ContractError::Unauthorized {}
    );

    let response = process_repayments(
        &info,
        &config,
        &mut state,
        &mut deps,
        cycle_profit,
        response,
    )?;

    Ok(response)
}

pub fn handle_cancel_redemption(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(map_to_contract_error)?;

    let (response, deps) =
        process_management_fees_and_modify_response(deps, Response::default(), env.clone(), None)?;

    State::is_open_and_unpaused(deps.as_ref())?;

    let config = Config::get_from_storage(deps.as_ref())?;

    let amount_to_return = remove_from_queue(deps.storage, info.sender.clone())?;

    let token_to_return = coin(amount_to_return.u128(), config.strategy_denom);

    Ok(response
        .add_event(event_cancel_redemption(
            info.sender.as_str(),
            token_to_return.clone(),
        ))
        .add_message(create_bank_message(
            info.sender.to_string(),
            vec![token_to_return],
        )))
}

pub fn handle_create_redemption(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _amount: Uint128, // not used when sending funds
) -> Result<Response, ContractError> {
    let (response, deps) =
        process_management_fees_and_modify_response(deps, Response::default(), env.clone(), None)?;

    State::is_open_and_unpaused(deps.as_ref())?;

    let config = Config::get_from_storage(deps.as_ref())?;

    let strategy_denom_sent =
        must_pay(&info, &config.strategy_denom).map_err(|_| ContractError::InvalidFunds {})?;

    add_to_queue(
        deps.storage,
        info.sender.clone(),
        strategy_denom_sent,
        env.block.time.seconds(),
    )?;

    Ok(response.add_event(event_create_redemption(
        info.sender.as_str(),
        coin(strategy_denom_sent.u128(), config.strategy_denom),
    )))
}

pub fn handle_repay_queue(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cycle_profit: Option<Decimal>,
    max_queue_amount: Option<u64>,
) -> Result<Response, ContractError> {
    // 1. Process management fees and ensure the state is open and unpaused
    let (mut response, mut deps) =
        process_management_fees_and_modify_response(deps, Response::default(), env.clone(), None)?;

    State::is_open_and_unpaused(deps.as_ref())?;

    // 2. Load configuration and state immutably
    let config = Config::get_from_storage(deps.as_ref())?;
    let mut state = State::get_from_storage(deps.as_ref())?;

    // 3. Check authorization
    ensure!(
        config.controller == info.sender.to_string(),
        ContractError::Unauthorized {}
    );

    // 4. Process repayments (mutable `deps` is passed here)
    response = process_repayments(
        &info,
        &config,
        &mut state,
        &mut deps,
        cycle_profit,
        response,
    )?;

    // 5. Immutable borrow: Get the vault balance and then make a mutable copy to track remaining balance
    let vault_balance = get_vault_balance(&deps.as_ref(), &config, env.contract.address.as_str())?;
    let mut remaining_balance = vault_balance;

    // 6. Store redemptions to process
    let mut redemptions_to_process = vec![];

    // 7. Process redemptions
    let limit = max_queue_amount.unwrap_or(50);

    for result in redemptions()
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit as usize)
    {
        let (_, redemption) = result?;

        // Immutable operations
        let strategy_denom_sent = redemption.total_deposits;

        // Immutable operations done here
        let (burn_ratio, assets_to_redeem) = calculate_share_to_burn(
            deps.as_ref(),
            &config,
            &state,
            info.sender.as_str(),
            env.contract.address.as_str(),
            strategy_denom_sent,
        )?;

        match calculate_total_assets_redeemable(&assets_to_redeem, &mut remaining_balance) {
            Result::Ok(_) => {}
            Result::Err(_) => {
                break;
            }
        }

        // Add the redemption to the list of redemptions to process
        redemptions_to_process.push((redemption, burn_ratio, assets_to_redeem));
    }

    for redemption in redemptions_to_process.iter() {
        let (redemption, burn_ratio, assets_to_redeem) = redemption;

        // Update user deposit
        update_user_deposit(
            deps.storage,
            redemption.user.clone(),
            *burn_ratio,
            &mut state,
        )?;

        // Save state
        state.save_to_storage(&mut deps)?;

        // Process redeem
        response = process_redeem(
            response,
            &info,
            assets_to_redeem.to_vec(),
            &config,
            &env,
            redemption.total_deposits,
        )?;

        // Remove the redemption from the queue
        remove_from_queue(deps.storage, redemption.user.clone())?;
    }

    Ok(response)
}

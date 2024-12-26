use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    events::{
        event_cancel_redemption, event_create_redemption, event_fees, event_repay, event_withdraw,
    },
    helpers::{
        calculate_amount_withdrawable, calculate_assets_to_redeem, check_is_valid_token,
        ensure_no_duplicate_denoms, get_sent_tokens, get_vault_balance, map_to_contract_error,
    },
    messages::create_bank_message,
    process::{process_management_fees_and_modify_response, process_redeem, process_repayments},
    queries::{get_balance, get_total_supply},
    storage::{
        config::Config,
        queue::{add_to_queue, redemptions, remove_from_queue},
        state::{update_user_deposit, State},
    },
};

use cosmwasm_std::{
    coin, ensure, Coin, Decimal, Deps, DepsMut, Env, MessageInfo, Order, Response, Uint128,
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
    max_queue_amount: Option<Uint128>,
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
    for result in redemptions().range(deps.storage, None, None, Order::Ascending) {
        let (_, redemption) = result?;

        // Immutable operations
        let strategy_denom_sent = redemption.total_deposits;

        // Immutable operations done here
        let (burn_ratio, assets_to_redeem) = get_assets_to_burn(
            deps.as_ref(),
            &config,
            &state,
            info.sender.as_str(),
            env.contract.address.as_str(),
            strategy_denom_sent,
        )?;

        let mut insufficient_balance = false;

        // Iterate through the assets to redeem and ensure we have sufficient balance to fulfill the redemption
        for asset in assets_to_redeem.iter() {
            if let Some(balance) = remaining_balance
                .iter_mut()
                .find(|c| c.denom == asset.denom)
            {
                // Check that this redemption isn't larger than the available balance
                if asset.amount > balance.amount {
                    insufficient_balance = true;
                    break;
                }

                // Subtract the amounts (update the balance in-place)
                balance.amount = balance.amount.saturating_sub(asset.amount);
            } else {
                // If the asset doesn't exist in remaining_balance, handle this as an error case
                insufficient_balance = true;
                break;
            }
        }

        if insufficient_balance {
            break;
        }

        // Add the redemption to the list of redemptions to process
        redemptions_to_process.push((redemption, burn_ratio, assets_to_redeem));
    }
    // // Mutable operations: Scoped separately to avoid conflicts
    // {
    //     // Update user deposit (mutable borrow starts)
    //     update_user_deposit(
    //         deps.storage,
    //         redemption.sender.clone(),
    //         burn_ratio,
    //         &mut state,
    //     )?;
    // }

    // //    // Mutable operations: Scoped separately to avoid conflicts
    // //    {
    // //     // Update user deposit (mutable borrow starts)
    // //     update_user_deposit(deps.storage, info.sender.clone(), burn_ratio, &mut state)?;
    // // }

    // // // Save updated state to storage: Scoped separately
    // // {
    // //     state.save_to_storage(&mut deps)?;
    // // }

    // // // Redeem the assets (no mutable borrow here)
    // // response = process_redeem(
    // //     response,
    // //     &info,
    // //     assets_to_redeem,
    // //     &config,
    // //     &env,
    // //     strategy_denom_sent,
    // // )?;

    Ok(response)
}

pub fn get_assets_to_burn(
    deps: Deps,
    config: &Config,
    state: &State,
    sender: &str,
    contract: &str,
    amount_sent: Uint128,
) -> Result<(Decimal, Vec<Coin>), ContractError> {
    let user_vault_token_balance = get_balance(&deps, sender, &config.strategy_denom)?;

    let total_user_vault_token_balance = user_vault_token_balance
        .checked_add(amount_sent)
        .map_err(ContractError::Overflow)?;

    let total_supply = get_total_supply(&deps, &config.strategy_denom)?;

    let withdraw_percentage = Decimal::from_ratio(amount_sent, total_supply);

    let assets_to_redeem =
        calculate_assets_to_redeem(&deps, config, state, contract, withdraw_percentage)?;

    let burn_ratio = Decimal::from_ratio(amount_sent, total_user_vault_token_balance);

    Ok((burn_ratio, assets_to_redeem))
}

use crate::{
    errors::ContractError,
    events::{event_claim_redemption, event_send_redemption, event_update_config},
    storage::{
        redemptions::{add_to_pending, filter_user_redemptions, remove_from_pending},
        state::CONFIG,
    },
    utils::map_to_contract_error,
};

use cosmwasm_std::{ensure, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_utils::nonpayable;
use interface::redemption::{FundInfo, PendingRedemption};
use std::collections::HashMap;

fn validate_whitelisted_fund(
    whitelisted_funds: &[FundInfo],
    source: &str,
) -> Result<(), ContractError> {
    ensure!(
        whitelisted_funds.iter().any(|f| f.address == source),
        ContractError::UnauthorizedFund {}
    );
    Ok(())
}

fn validate_sent_funds(sent: &[Coin], expected: &[Coin]) -> Result<(), ContractError> {
    let mut sent_map: HashMap<String, u128> = HashMap::new();
    for coin in sent {
        sent_map.insert(coin.denom.clone(), coin.amount.u128());
    }

    for coin in expected {
        match sent_map.get(&coin.denom) {
            Some(&amount) if amount == coin.amount.u128() => continue,
            _ => return Err(ContractError::InsufficientFunds {}),
        }
    }

    Ok(())
}

pub fn handle_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    add_fund: Option<FundInfo>,
    remove_fund: Option<FundInfo>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only admin can update config
    ensure!(
        config.admin == info.sender.to_string(),
        ContractError::Unauthorized {}
    );

    // Handle fund addition
    if let Some(fund) = add_fund.clone() {
        config.whitelisted_funds.push(fund);
    }

    // Handle fund removal
    if let Some(fund) = remove_fund.clone() {
        config
            .whitelisted_funds
            .retain(|f| f.address != fund.address);
    }

    // Validate the updated config
    config.validate(&deps.as_ref())?;

    // Save the updated config
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_event(event_update_config(add_fund, remove_fund)))
}

pub fn handle_send_redemption(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut redemption: PendingRedemption,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Validate the source is a whitelisted fund
    validate_whitelisted_fund(&config.whitelisted_funds, info.sender.as_str())?;

    // Validate that sent funds match redemption funds
    validate_sent_funds(&info.funds, &redemption.funds)?;

    // Set the timestamp to the current block time
    redemption.timestamp = env.block.time.seconds();

    // Add the redemption to pending
    add_to_pending(deps.storage, redemption.clone())?;

    Ok(Response::new().add_event(event_send_redemption(redemption)))
}

pub fn handle_claim_redemption(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    limit: Option<u32>,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(map_to_contract_error)?;

    let user = info.sender.to_string();
    let limit = limit.unwrap_or(10).min(30) as usize;

    // First collect all redemptions to claim
    let redemptions_to_claim: Vec<(u64, PendingRedemption)> =
        filter_user_redemptions(deps.storage, user.clone())
            .take(limit)
            .map(|result| result.map(|((_, timestamp), redemption)| (timestamp, redemption)))
            .collect::<StdResult<Vec<_>>>()?;

    ensure!(
        !redemptions_to_claim.is_empty(),
        ContractError::NoRedemptionsFound {}
    );

    // Now remove the redemptions and collect bank messages
    let mut claimed_redemptions = Vec::with_capacity(redemptions_to_claim.len());
    let mut bank_msgs = Vec::with_capacity(redemptions_to_claim.len());

    for (timestamp, redemption) in redemptions_to_claim {
        remove_from_pending(deps.storage, user.clone(), timestamp)?;
        claimed_redemptions.push(redemption.clone());

        // Create bank send message for this redemption
        bank_msgs.push(BankMsg::Send {
            to_address: redemption.user,
            amount: redemption.funds,
        });
    }

    Ok(Response::new()
        .add_event(event_claim_redemption(claimed_redemptions))
        .add_messages(bank_msgs))
}

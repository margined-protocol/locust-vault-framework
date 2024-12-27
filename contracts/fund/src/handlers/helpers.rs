use crate::{
    helpers::calculate_assets_to_redeem,
    queries::external::{get_balance, get_total_supply},
    storage::{config::Config, state::State},
};

use cosmwasm_std::{Coin, Decimal, Deps, StdError, Uint128};
use vaultenator::errors::ContractError;

pub fn calculate_total_assets_redeemable(
    assets_to_redeem: &[Coin],
    remaining_balance: &mut [Coin],
) -> Result<(), StdError> {
    for asset in assets_to_redeem.iter() {
        if let Some(balance) = remaining_balance
            .iter_mut()
            .find(|c| c.denom == asset.denom)
        {
            // Check that this redemption isn't larger than the available balance
            if asset.amount > balance.amount {
                return Err(StdError::generic_err(format!(
                    "Insufficient balance for denom '{}'. Required: {}, Available: {}",
                    asset.denom, asset.amount, balance.amount
                )));
            }

            // Subtract the amounts (update the balance in-place)
            balance.amount = balance.amount.saturating_sub(asset.amount);
        } else {
            // If the asset doesn't exist in remaining_balance, return an error
            return Err(StdError::generic_err(format!(
                "Denomination '{}' not found in remaining balance.",
                asset.denom
            )));
        }
    }

    Ok(())
}

pub fn calculate_share_to_burn(
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

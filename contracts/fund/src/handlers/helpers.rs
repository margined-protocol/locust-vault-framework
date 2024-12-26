use cosmwasm_std::{Coin, StdError};

pub fn process_assets_to_redeem(
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

use crate::{
    config::Config,
    events::event_sudo,
    queries::get_balance,
    state::{UserDeposit, USER_DEPOSITS},
};

use cosmwasm_std::{ensure, Coin, Decimal, Deps, DepsMut, Env, Response, StdError, Uint128};
use vaultenator::{config::Configure, errors::ContractError};

pub const NEUTRON_TOKEN_FACTORY_ADDRESS: &str = "neutron19ejy8n9qsectrf4semdp9cpknflld0j6el50hx";
pub const OSMOSIS_TOKEN_FACTORY_ADDRESS: &str = "osmo19ejy8n9qsectrf4semdp9cpknflld0j64mwamn";

pub fn sudo_block_before_send(
    deps: DepsMut,
    env: Env,
    from: String,
    to: String,
    sent: Coin,
) -> Result<Response, ContractError> {
    // Early return if 'from' or 'to' are exempt addresses
    if is_exempt_address(&from, env.contract.address.as_str())
        || is_exempt_address(&to, env.contract.address.as_str())
    {
        return Ok(Response::default());
    }

    // Retrieve configuration and validate addresses
    let config = Config::get_from_storage(deps.as_ref())?;
    let sender = deps.api.addr_validate(&from)?;
    let receiver = deps.api.addr_validate(&to)?;

    // check that the sent denom is correct
    ensure!(
        sent.denom == config.strategy_denom,
        ContractError::Std(StdError::generic_err(
            "Invalid sent denom, must be strategy denom"
        ))
    );

    // Load deposits for sender and receiver
    let mut sender_deposit = USER_DEPOSITS.load(deps.storage, sender.clone())?;
    let mut receiver_deposit = USER_DEPOSITS
        .may_load(deps.storage, receiver.clone())?
        .unwrap_or_else(|| UserDeposit::empty_deposit(env.block.time.seconds()));

    // Calculate burn ratio and adjust deposits
    calculate_and_adjust_deposits(
        &deps.as_ref(),
        &config,
        &from,
        sent.amount,
        &mut sender_deposit,
        &mut receiver_deposit,
    )?;

    // Save updated deposits
    USER_DEPOSITS.save(deps.storage, sender, &sender_deposit)?;
    USER_DEPOSITS.save(deps.storage, receiver, &receiver_deposit)?;

    Ok(Response::default().add_event(event_sudo(&to, &from, &sent)))
}

fn calculate_and_adjust_deposits(
    deps: &Deps,
    config: &Config,
    from: &str,
    sent_amount: Uint128,
    sender_deposit: &mut UserDeposit,
    receiver_deposit: &mut UserDeposit,
) -> Result<(), ContractError> {
    let user_vault_token_balance = get_balance(deps, from, &config.strategy_denom)?;

    let burn_ratio = Decimal::from_ratio(sent_amount, user_vault_token_balance);
    let sender_deposit_sent = sender_deposit.total_deposits * burn_ratio;

    sender_deposit.remove_from_user_deposits(sender_deposit_sent)?;
    receiver_deposit.add_to_user_deposits(sender_deposit_sent)?;

    Ok(())
}

fn is_exempt_address(address: &str, contract_address: &str) -> bool {
    const EXEMPT_ADDRESSES: [&str; 2] =
        [OSMOSIS_TOKEN_FACTORY_ADDRESS, NEUTRON_TOKEN_FACTORY_ADDRESS];
    EXEMPT_ADDRESSES.contains(&address) || address == contract_address
}

use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    helpers::coins_to_string,
};

use cosmwasm_std::{Coin, Event};
use cw2::ContractVersion;

pub fn event_withdraw(user: &str, amount_withdrawn: Coin) -> Event {
    Event::new("withdraw").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("user", user),
        ("amount_withdrawn", &amount_withdrawn.to_string()),
    ])
}

pub fn event_cancel_redemption(user: &str, token_in: Coin) -> Event {
    Event::new("cancel_redemption").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("user", user),
        ("amount", &token_in.to_string()),
    ])
}

pub fn event_create_redemption(user: &str, token_in: Coin) -> Event {
    Event::new("create_redemption").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("user", user),
        ("amount", &token_in.to_string()),
    ])
}

pub fn event_repay(user: &str, token_in: Coin) -> Event {
    Event::new("repay").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("user", user),
        ("amount_repayed", &token_in.to_string()),
    ])
}

pub fn event_repay_and_process(user: &str, token_in: Coin, redemption_out: Coin) -> Event {
    Event::new("repay_and_process").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("user", user),
        ("amount_repayed", &token_in.to_string()),
        ("amount_processed", &redemption_out.to_string()),
    ])
}

pub fn event_sudo(to: &str, from: &str, amount: &Coin) -> Event {
    Event::new("sudo").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("to", to),
        ("from", from),
        ("amount", &amount.to_string()),
    ])
}

pub fn event_burn(
    version: &str,
    name: &str,
    user: &str,
    amount: &str,
    penalty: Option<&str>,
) -> Event {
    Event::new("burn").add_attributes([
        ("version", version),
        ("contract", name),
        ("user", user),
        ("amount", amount),
        (
            "penalty",
            &penalty.map_or("-".to_string(), |penalty| penalty.to_string()),
        ),
    ])
}

pub fn event_mint(version: &str, name: &str, user: &str, amount: &str) -> Event {
    Event::new("mint").add_attributes([
        ("version", version),
        ("contract", name),
        ("user", user),
        ("amount", amount),
    ])
}

pub fn event_deposit(
    version: &str,
    name: &str,
    user: &str,
    token0: Coin,
    token1: Option<Coin>,
) -> Event {
    Event::new("deposit").add_attributes([
        ("version", version),
        ("contract", name),
        ("user", user),
        ("token0", &token0.to_string()),
        (
            "token1",
            &token1.map_or("-".to_string(), |coin| coin.to_string()),
        ),
    ])
}

pub fn event_fees(version: &str, name: &str, fee_type: &str, fees: Vec<Coin>) -> Event {
    Event::new("fees").add_attributes([
        ("version", version),
        ("contract", name),
        ("fee_type", fee_type),
        ("fees", &coins_to_string(fees)),
    ])
}

pub fn event_redeem(
    version: &str,
    name: &str,
    user: &str,
    token0: Coin,
    token1: Option<Coin>,
    fee0: Coin,
    fee1: Option<Coin>,
) -> Event {
    Event::new("redeem").add_attributes([
        ("version", version),
        ("contract", name),
        ("user", user),
        ("token0", &token0.to_string()),
        (
            "token1",
            &token1.map_or("-".to_string(), |coin| coin.to_string()),
        ),
        ("token0_fees", &fee0.to_string()),
        (
            "token1_fees",
            &fee1.map_or("-".to_string(), |coin| coin.to_string()),
        ),
    ])
}

pub fn event_register_sudo() -> Event {
    Event::new("register_sudo")
        .add_attributes([("version", CONTRACT_VERSION), ("contract", CONTRACT_NAME)])
}

pub fn event_migrate(version: &str, name: &str, contract_version: ContractVersion) -> Event {
    Event::new("migrate").add_attributes([
        ("previous_contract_name", &contract_version.contract),
        ("previous_contract_version", &contract_version.version),
        ("new_contract_name", &format!("crates.io:{name}")),
        ("new_contract_version", &version.to_string()),
    ])
}

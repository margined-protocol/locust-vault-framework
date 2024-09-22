use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_std::Event;

pub fn event_withdraw(withdraw: String) -> Event {
    Event::new("withdraw").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("withdraw", &withdraw),
    ])
}

pub fn event_repay(repay: String) -> Event {
    Event::new("repay").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("repay", &repay),
    ])
}

pub fn event_set_vault(vault: String) -> Event {
    Event::new("set_vault").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("vault", &vault),
    ])
}

pub fn event_set_grants(grants: Vec<String>) -> Event {
    Event::new("set_grants").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("grants", &grants.join(",")),
    ])
}

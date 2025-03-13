use crate::contract::{CONTRACT_NAME, CONTRACT_VERSION};

use cosmwasm_std::Event;
use cw2::ContractVersion;
use neutron_std::types::cosmos::bank::v1beta1::SendAuthorization;

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

pub fn event_set_send_authorization(send_authorization: SendAuthorization) -> Event {
    Event::new("set_send_authorization").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("allow_list", &send_authorization.allow_list[0].to_string()),
    ])
}

pub fn event_update_config(grants: Option<Vec<String>>, controller: Option<String>) -> Event {
    Event::new("update_config").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        ("grants", &grants.unwrap_or_default().join(",")),
        ("controller", &controller.unwrap_or_default()),
    ])
}

pub fn event_migrate(version: &str, name: &str, contract_version: ContractVersion) -> Event {
    Event::new("migrate").add_attributes([
        ("previous_contract_name", &contract_version.contract),
        ("previous_contract_version", &contract_version.version),
        ("new_contract_name", &format!("crates.io:{name}")),
        ("new_contract_version", &version.to_string()),
    ])
}

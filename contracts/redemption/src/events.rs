use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    utils::tokens_to_string,
};

use cosmwasm_std::Event;
use cw2::ContractVersion;
use interface::redemption::{FundInfo, PendingRedemption};

pub fn event_send_redemption(redemption: PendingRedemption) -> Event {
    Event::new("send_redemption")
        .add_attribute("user", redemption.user)
        .add_attribute("timestamp", redemption.timestamp.to_string())
        .add_attribute("source", redemption.source)
        .add_attribute("funds", tokens_to_string(redemption.funds))
}

pub fn event_claim_redemption(redemptions: Vec<PendingRedemption>) -> Event {
    Event::new("claim_redemption")
        .add_attribute("user", redemptions[0].user.clone())
        .add_attribute("count", redemptions.len().to_string())
        .add_attribute(
            "total_claimed",
            redemptions
                .iter()
                .map(|r| r.funds.iter().map(|c| c.amount.u128()).sum::<u128>())
                .sum::<u128>()
                .to_string(),
        )
}

pub fn event_update_config(add_fund: Option<FundInfo>, remove_fund: Option<FundInfo>) -> Event {
    Event::new("update_config").add_attributes([
        ("version", CONTRACT_VERSION),
        ("contract", CONTRACT_NAME),
        (
            "add_fund",
            &add_fund.map_or_else(String::new, |f| f.to_string()),
        ),
        (
            "remove_fund",
            &remove_fund.map_or_else(String::new, |f| f.to_string()),
        ),
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

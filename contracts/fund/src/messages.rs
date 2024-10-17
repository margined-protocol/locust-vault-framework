use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Uint128};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmosisCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
};

pub fn create_bank_message(to_address: String, amount: Vec<Coin>) -> CosmosMsg {
    BankMsg::Send { to_address, amount }.into()
}

pub fn create_burn_message(
    contract_address: &Addr,
    burn_from: String,
    amount: Uint128,
    denom: String,
) -> CosmosMsg {
    MsgBurn {
        sender: contract_address.to_string(),
        amount: Some(OsmosisCoin {
            denom,
            amount: amount.to_string(),
        }),
        burn_from_address: burn_from.to_string(),
    }
    .into()
}

pub fn create_mint_message(
    contract_address: &Addr,
    recipient: String,
    amount: Uint128,
    denom: String,
) -> CosmosMsg {
    MsgMint {
        sender: contract_address.to_string(),
        amount: Some(OsmosisCoin {
            denom,
            amount: amount.to_string(),
        }),
        mint_to_address: recipient.to_string(),
    }
    .into()
}

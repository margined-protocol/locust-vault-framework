use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Uint128, WasmMsg};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmosisCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint},
};

#[cw_serde]
pub struct PendingRedemption {
    pub user: String,
    pub funds: Vec<Coin>,
    pub timestamp: u64,
    pub source: String,
}

#[cw_serde]
pub enum RedemptionExecuteMsg {
    SendRedemption {
        redemption: PendingRedemption, // Redemptions must be sent individually
    },
}

pub fn create_bank_message(to_address: String, amount: Vec<Coin>) -> CosmosMsg {
    BankMsg::Send { to_address, amount }.into()
}

pub fn create_redemption_message(
    contract: String,
    user: String,
    funds: Vec<Coin>,
    timestamp: u64,
    source: String,
) -> CosmosMsg {
    WasmMsg::Execute {
        contract_addr: contract,
        msg: to_json_binary(&RedemptionExecuteMsg::SendRedemption {
            redemption: PendingRedemption {
                user,
                funds: funds.clone(),
                timestamp,
                source,
            },
        })
        .unwrap(),
        funds,
    }
    .into()
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

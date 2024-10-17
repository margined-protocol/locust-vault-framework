use crate::{config::Config, contract::StructuredVault, messages::create_mint_message};

use cosmwasm_std::{DepsMut, Env, Reply, Response, SubMsgResult, Uint128};
use num_enum::TryFromPrimitive;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;
use strum::IntoStaticStr;
use vaultenator::{errors::ContractError, reply::ReplyHandler};

pub const INITIAL_TOKEN_SUPPLY: Uint128 = Uint128::new(1_000_000u128);

#[derive(TryFromPrimitive, IntoStaticStr, Debug, Clone, PartialEq)]
#[repr(u64)]
pub enum ReplyIDs {
    CreateStrategyDenom,
}

impl ReplyHandler<Config> for StructuredVault {
    fn handle_reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        let reply_id = ReplyIDs::try_from(msg.id).map_err(|_| ContractError::InvalidReplyId)?;
        match reply_id {
            ReplyIDs::CreateStrategyDenom => reply_create_strategy_denom(deps, env, msg),
        }
    }
}

pub fn reply_create_strategy_denom(
    _deps: DepsMut,
    env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    let sub_msg_response: SubMsgResult = msg.result;
    let response: MsgCreateDenomResponse = sub_msg_response.try_into()?;

    // Set mint_to_address to recipient if set, sender if not
    let mint_msg = create_mint_message(
        &env.contract.address,
        env.contract.address.to_string(),
        INITIAL_TOKEN_SUPPLY,
        response.new_token_denom.to_string(),
    );

    Ok(Response::default()
        .add_attribute("strategy_denom", &response.new_token_denom)
        .add_message(mint_msg))
}

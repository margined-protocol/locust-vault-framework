use cosmwasm_std::{Binary, Coin, CosmosMsg};
use osmosis_std::{
    shim::Any,
    types::cosmos::authz::v1beta1::{GenericAuthorization, Grant, MsgGrant},
};
use prost::Message;

pub fn tokens_to_string(tokens: Vec<Coin>) -> String {
    tokens
        .iter()
        .map(|coin| format!("{}{}", coin.amount, coin.denom))
        .collect::<Vec<String>>()
        .join(",")
}

pub fn create_authz_grant_messages(
    granter: &str,
    grantee: &str,
    message_types: &[&str],
) -> Vec<CosmosMsg> {
    message_types
        .iter()
        .map(|msg_type| {
            let mut buf = vec![];
            GenericAuthorization {
                msg: msg_type.to_string(),
            }
            .encode(&mut buf)
            .unwrap();

            CosmosMsg::Stargate {
                type_url: MsgGrant::TYPE_URL.to_string(),
                value: Binary::from(
                    MsgGrant {
                        granter: granter.to_string(),
                        grantee: grantee.to_string(),
                        grant: Some(Grant {
                            authorization: Some(Any {
                                type_url: GenericAuthorization::TYPE_URL.to_string(),
                                value: buf,
                            }),
                            expiration: None,
                        }),
                    }
                    .encode_to_vec(),
                ),
            }
        })
        .collect()
}

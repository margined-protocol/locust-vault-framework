use crate::setup::TestEnv;

use cosmwasm_std::Uint128;
use neutron_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin};
use neutron_test_tube::{
    neutron_std::types::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryTotalSupplyRequest},
    Account, Bank, Module, SigningAccount,
};
// use osmosis_std::types::{
//     cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin as OsmoCoin},
//     osmosis::poolmanager::v1beta1::{PoolRequest, SpotPriceRequest},
// };
// use osmosis_test_tube::{
//     cosmrs::proto::traits::Message,
//     osmosis_std::types::{
//         cosmos::bank::v1beta1::{QueryBalanceRequest, QueryTotalSupplyRequest},
//         osmosis::concentratedliquidity::v1beta1::Pool,
//     },
//     Account, Bank, Module, PoolManager, SigningAccount,
// };
use std::str::FromStr;

impl TestEnv {
    pub fn send(&self, to_address: &str, amount: OsmoCoin, sender: &SigningAccount) {
        let bank = Bank::new(&self.app);

        bank.send(
            MsgSend {
                from_address: sender.address(),
                to_address: to_address.to_string(),
                amount: vec![amount],
            },
            sender,
        )
        .unwrap();
    }

    pub fn get_balance(&self, address: &str, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address: address.to_string(),
                denom: denom.to_string(),
            })
            .unwrap();

        match response.balance {
            Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
            None => Uint128::zero(),
        }
    }

    // pub fn get_pool(&self, pool_id: u64) -> Pool {
    //     let poolmanager = PoolManager::new(&self.app);
    //     let res = poolmanager.query_pool(&PoolRequest { pool_id }).unwrap();
    //     Pool::decode(res.pool.unwrap().value.as_ref()).unwrap()
    // }

    // pub fn get_spot_price(
    //     &self,
    //     pool_id: u64,
    //     base_asset_denom: &str,
    //     quote_asset_denom: &str,
    // ) -> Decimal {
    //     let pm = PoolManager::new(&self.app);

    //     let res = pm
    //         .query_spot_price(&SpotPriceRequest {
    //             pool_id,
    //             base_asset_denom: base_asset_denom.to_string(),
    //             quote_asset_denom: quote_asset_denom.to_string(),
    //         })
    //         .unwrap()
    //         .spot_price;

    //     Decimal::from_str(&res).unwrap()
    // }

    pub fn get_total_supply(&self, denom: &str) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_total_supply(&QueryTotalSupplyRequest { pagination: None })
            .unwrap()
            .supply
            .into_iter()
            .find(|coin| coin.denom == denom)
            .unwrap();

        Uint128::from_str(&response.amount).unwrap_or(Uint128::zero())
    }
}

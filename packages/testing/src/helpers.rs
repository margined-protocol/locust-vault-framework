use crate::{
    setup::{TestEnv, BASE_DENOM, QUOTE_DENOM, TICK_SPACING},
    utils::store_code,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal, SignedDecimal, Uint128};
use cw_margined::math::price_to_tick;
use interface::strategy::{InstantiateMsg, PoolInfo};
use osmosis_std::{
    shim::{Any, Timestamp},
    types::{
        cosmos::{
            authz::v1beta1::{MsgExec, MsgExecResponse},
            bank::v1beta1::MsgSend,
            base::v1beta1::Coin as OsmoCoin,
        },
        cosmwasm::wasm::v1::MsgInstantiateContractResponse,
        osmosis::{
            concentratedliquidity::v1beta1::{
                MsgCreatePosition, MsgWithdrawPosition, UserPositionsRequest, UserPositionsResponse,
            },
            poolmanager::v1beta1::{
                MsgSwapExactAmountIn, MsgSwapExactAmountInResponse, MsgSwapExactAmountOut,
                MsgSwapExactAmountOutResponse, PoolRequest, SpotPriceRequest, SwapAmountInRoute,
                SwapAmountOutRoute,
            },
            twap::v1beta1::ArithmeticTwapToNowRequest,
        },
    },
};
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::{
        cosmos::bank::v1beta1::{QueryBalanceRequest, QueryTotalSupplyRequest},
        osmosis::concentratedliquidity::v1beta1::Pool,
    },
    Account, Authz, Bank, ConcentratedLiquidity, ExecuteResponse, Module, OsmosisTestApp,
    PoolManager, RunnerExecuteResult, SigningAccount, Twap, Wasm,
};
use serde::Serialize;
use std::str::FromStr;

#[cw_serde]
pub struct MockInstantiateMsg {}

pub fn get_default_instantiation_msg(env: &TestEnv) -> InstantiateMsg {
    InstantiateMsg {
        controller: env.controller.address(),
        vault: env.signer.address(),
        token0: BASE_DENOM.to_string(),
        token1: None,
        grants: vec![MsgCreatePosition::TYPE_URL.to_string()],
        pool_info: PoolInfo::Osmosis {
            id: 1,
            token0: BASE_DENOM.to_string(),
            token1: QUOTE_DENOM.to_string(),
        },
    }
}

impl TestEnv {
    pub fn instantiate_contract<M>(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: &M,
        funds: Vec<Coin>,
        signer: &SigningAccount,
        contract_name: &str,
    ) -> RunnerExecuteResult<MsgInstantiateContractResponse>
    where
        M: Serialize,
    {
        let code_id = store_code(wasm, signer, contract_name).unwrap();
        wasm.instantiate(
            code_id,
            msg,
            Some(&self.signer.address()),
            Some("vault"),
            &funds,
            signer,
        )
    }

    pub fn deploy_strategy_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        msg: Option<InstantiateMsg>,
    ) -> String {
        let msg = msg.unwrap_or_else(|| get_default_instantiation_msg(self));

        self.instantiate_contract(wasm, &msg, vec![], &self.signer, "strategy")
            .unwrap()
            .data
            .address
    }

    pub fn deploy_mock_vault(&self, wasm: &Wasm<OsmosisTestApp>) -> String {
        self.instantiate_contract(
            wasm,
            &MockInstantiateMsg {},
            vec![],
            &self.signer,
            "mock_vault",
        )
        .unwrap()
        .data
        .address
    }

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

    pub fn create_lsd_positions(
        &self,
        prices: [Decimal; 2],
        spread: Decimal,
        assets: [Coin; 2],
        granter: &str,
        signer: &SigningAccount,
        pool_id: u64,
    ) {
        let authz = Authz::new(&self.app);

        let lower_price = prices[0];
        let upper_price = prices[1];

        let lower_bound = lower_price * (Decimal::one() - spread);
        let upper_bound = upper_price * (Decimal::one() + spread);

        let base_amount = assets[0].clone();
        let quote_amount = assets[1].clone();

        let quote_position_lower = price_to_tick(
            SignedDecimal::from_str(&lower_bound.to_string()).unwrap(),
            TICK_SPACING as u32,
        )
        .unwrap();
        let quote_position_upper = price_to_tick(
            SignedDecimal::from_str(&lower_price.to_string()).unwrap(),
            TICK_SPACING as u32,
        )
        .unwrap();

        let base_position_lower = price_to_tick(
            SignedDecimal::from_str(&upper_price.to_string()).unwrap(),
            TICK_SPACING as u32,
        )
        .unwrap();
        let base_position_upper = price_to_tick(
            SignedDecimal::from_str(&upper_bound.to_string()).unwrap(),
            TICK_SPACING as u32,
        )
        .unwrap();

        let mut quote_buf = vec![];
        MsgCreatePosition::encode(
            &MsgCreatePosition {
                sender: granter.to_string(),
                pool_id,
                lower_tick: quote_position_lower,
                upper_tick: quote_position_upper,
                tokens_provided: vec![quote_amount.clone().into()],
                token_min_amount0: "0".to_string(),
                token_min_amount1: "0".to_string(),
            },
            &mut quote_buf,
        )
        .unwrap();

        let mut base_buf = vec![];
        MsgCreatePosition::encode(
            &MsgCreatePosition {
                sender: granter.to_string(),
                pool_id,
                lower_tick: base_position_lower,
                upper_tick: base_position_upper,
                tokens_provided: vec![base_amount.clone().into()],
                token_min_amount0: "0".to_string(),
                token_min_amount1: "0".to_string(),
            },
            &mut base_buf,
        )
        .unwrap();

        let mut msgs = Vec::new();

        if !base_amount.amount.is_zero() {
            msgs.push(Any {
                type_url: MsgCreatePosition::TYPE_URL.to_string(),
                value: base_buf,
            });
        }

        if !quote_amount.amount.is_zero() {
            msgs.push(Any {
                type_url: MsgCreatePosition::TYPE_URL.to_string(),
                value: quote_buf,
            });
        }

        authz
            .exec(
                MsgExec {
                    grantee: signer.address().to_string(),
                    msgs,
                },
                signer,
            )
            .unwrap();
    }

    pub fn withdraw_cl_positions(
        &self,
        granter: String,
        signer: &SigningAccount,
        pool_id: u64,
    ) -> ExecuteResponse<MsgExecResponse> {
        let authz = Authz::new(&self.app);

        let positions = self.get_user_positions(&granter, pool_id);

        let mut msgs = vec![];
        for el in positions.positions {
            let position = el.position.unwrap();
            let mut buf = vec![];
            MsgWithdrawPosition::encode(
                &MsgWithdrawPosition {
                    sender: granter.clone(),
                    position_id: position.position_id,
                    liquidity_amount: position.liquidity,
                },
                &mut buf,
            )
            .unwrap();

            msgs.push(Any {
                type_url: MsgWithdrawPosition::TYPE_URL.to_string(),
                value: buf,
            });
        }

        authz
            .exec(
                MsgExec {
                    grantee: signer.address().to_string(),
                    msgs,
                },
                signer,
            )
            .unwrap()
    }

    pub fn swap_amount_in(
        &self,
        pool_id: u64,
        liquidity_to_sell: Uint128,
        token_in_denom: String,
        token_out_denom: String,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgSwapExactAmountInResponse> {
        let pool_manager = PoolManager::new(&self.app);
        pool_manager.swap_exact_amount_in(
            MsgSwapExactAmountIn {
                sender: signer.address(),
                routes: vec![SwapAmountInRoute {
                    pool_id,
                    token_out_denom,
                }],
                token_in: Some(OsmoCoin {
                    amount: liquidity_to_sell.to_string(),
                    denom: token_in_denom,
                }),
                token_out_min_amount: "1".to_string(),
            },
            signer,
        )
    }

    pub fn swap_amount_out(
        &self,
        pool_id: u64,
        liquidity_to_buy: Uint128,
        liquidity_to_sell: Uint128,
        token_in_denom: String,
        token_out_denom: String,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgSwapExactAmountOutResponse> {
        let pool_manager = PoolManager::new(&self.app);
        pool_manager.swap_exact_amount_out(
            MsgSwapExactAmountOut {
                sender: signer.address(),
                routes: vec![SwapAmountOutRoute {
                    pool_id,
                    token_in_denom,
                }],
                token_out: Some(OsmoCoin {
                    amount: liquidity_to_buy.to_string(),
                    denom: token_out_denom,
                }),
                token_in_max_amount: liquidity_to_sell.to_string(),
            },
            signer,
        )
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

    pub fn get_pool(&self, pool_id: u64) -> Pool {
        let poolmanager = PoolManager::new(&self.app);
        let res = poolmanager.query_pool(&PoolRequest { pool_id }).unwrap();
        Pool::decode(res.pool.unwrap().value.as_ref()).unwrap()
    }

    pub fn get_spot_price(
        &self,
        pool_id: u64,
        base_asset_denom: &str,
        quote_asset_denom: &str,
    ) -> Decimal {
        let pm = PoolManager::new(&self.app);

        let res = pm
            .query_spot_price(&SpotPriceRequest {
                pool_id,
                base_asset_denom: base_asset_denom.to_string(),
                quote_asset_denom: quote_asset_denom.to_string(),
            })
            .unwrap()
            .spot_price;

        Decimal::from_str(&res).unwrap()
    }

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

    pub fn get_twap(
        &self,
        pool_id: u64,
        base_asset_denom: &str,
        quote_asset_denom: &str,
        start_time_seconds: i64,
    ) -> Decimal {
        let tw = Twap::new(&self.app);

        let res = tw
            .query_arithmetic_twap_to_now(&ArithmeticTwapToNowRequest {
                pool_id,
                base_asset: base_asset_denom.to_string(),
                quote_asset: quote_asset_denom.to_string(),
                start_time: Some(Timestamp {
                    seconds: start_time_seconds,
                    nanos: 0,
                }),
            })
            .unwrap()
            .arithmetic_twap;

        let value = Uint128::from_str(&res).unwrap();

        Decimal::from_atomics(value, 18u32).unwrap()
    }

    pub fn get_user_positions(&self, address: &str, pool_id: u64) -> UserPositionsResponse {
        let cl = ConcentratedLiquidity::new(&self.app);

        cl.query_user_positions(&UserPositionsRequest {
            address: address.to_string(),
            pool_id,
            pagination: None,
        })
        .unwrap()
    }
}

use cosmwasm_std::{coin, Decimal, Uint128};
use osmosis_std::types::{
    cosmos::{
        base::v1beta1::Coin as OsmoCoin,
        params::v1beta1::{ParamChange, ParameterChangeProposal},
    },
    osmosis::concentratedliquidity::v1beta1::{MsgCreatePosition, PoolsRequest},
};
use osmosis_test_tube::{
    cosmrs::proto::traits::Message,
    osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
        CreateConcentratedLiquidityPoolsProposal, Pool, PoolRecord,
    },
    Account, ConcentratedLiquidity, GovWithAppAccess, Module, OsmosisTestApp, SigningAccount,
};
use std::str::FromStr;

pub const DEFAULT_LIQUIDITY: u128 = 1_000_000u128;
pub const PROPOSAL_DURATION: u64 = 1000;
pub const STRATEGY_CAP: Uint128 = Uint128::new(10_000_000_000_000u128); // 10,000 @6dp
pub const TICK_SPACING: u64 = 100;
pub const ONE_WEEK_IN_SECONDS: u64 = 604800;
pub const SPREAD_FACTOR: &str = "3000";
pub const GAS_DENOM: &str = "uosmo";
pub const BASE_DENOM: &str = "ubase";
pub const STAKE_DENOM: &str = "stbase";
pub const QUOTE_DENOM: &str = "usdc";
pub const REWARD_DENOM: &str = "ureward";

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub signer: SigningAccount,
    pub controller: SigningAccount,
    pub treasury: SigningAccount,
    pub traders: Vec<SigningAccount>,
    pub default_pool: Pool,
}

impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();
        let concentrated_liquidity = ConcentratedLiquidity::new(&app);
        let gov = GovWithAppAccess::new(&app);

        let signer = app
            .init_account(&[
                coin(1_000_000_000_000_000_000, BASE_DENOM),
                coin(1_000_000_000_000_000_000, STAKE_DENOM),
                coin(1_000_000_000_000_000_000, GAS_DENOM),
                coin(1_000_000_000_000_000, QUOTE_DENOM),
                coin(1_000_000_000_000_000, REWARD_DENOM),
            ])
            .unwrap();

        let controller = app
            .init_account(&[coin(1_000_000_000_000_000_000, GAS_DENOM)])
            .unwrap();

        let treasury = app.init_account(&[coin(1000, GAS_DENOM)]).unwrap();

        let mut traders: Vec<SigningAccount> = Vec::new();
        for _ in 0..10 {
            traders.push(
                app.init_account(&[
                    coin(1_000_000_000_000_000_000, BASE_DENOM),
                    coin(1_000_000_000_000_000_000, STAKE_DENOM),
                    coin(1_000_000_000_000_000_000, GAS_DENOM),
                    coin(1_000_000_000_000_000, QUOTE_DENOM),
                ])
                .unwrap(),
            );
        }

        // update the parameters so we can have no gas paying token as base
        gov.propose_and_execute(
            "/cosmos.params.v1beta1.ParameterChangeProposal".to_string(),
            ParameterChangeProposal {
                title: "Update authorized quote denoms".to_string(),
                description: "Add authorized quote denoms".to_string(),
                changes: vec![ParamChange {
                    subspace: "poolmanager".to_string(),
                    key: "AuthorizedQuoteDenoms".to_string(),
                    value: format!(
                        "[\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"]",
                        GAS_DENOM, BASE_DENOM, QUOTE_DENOM, REWARD_DENOM, STAKE_DENOM
                    ),
                }],
            },
            signer.address(),
            &signer,
        )
        .unwrap();

        create_cl_pool(
            &gov,
            &signer,
            BASE_DENOM.to_string(),
            QUOTE_DENOM.to_string(),
        );

        let pools = concentrated_liquidity
            .query_pools(&PoolsRequest { pagination: None })
            .unwrap();

        let default_pool = Pool::decode(pools.pools[0].value.as_slice()).unwrap();
        create_position(
            &app,
            &signer,
            default_pool.id,
            vec![
                OsmoCoin {
                    denom: BASE_DENOM.to_string(),
                    amount: "100000000000".to_string(),
                },
                OsmoCoin {
                    denom: QUOTE_DENOM.to_string(),
                    amount: "125000000000".to_string(),
                },
            ],
        );

        // move forward 1 day so that twaps will work
        app.increase_time(24 * 60 * 60);

        Self {
            app,
            signer,
            controller,
            treasury,
            traders,
            default_pool,
        }
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_cl_pool(
    gov: &GovWithAppAccess,
    signer: &SigningAccount,
    denom0: String,
    denom1: String,
) {
    gov.propose_and_execute(
        CreateConcentratedLiquidityPoolsProposal::TYPE_URL.to_string(),
        CreateConcentratedLiquidityPoolsProposal {
            title: "Create concentrated pool".to_string(),
            description: "Create concentrated pool, so that we can trade it".to_string(),
            pool_records: vec![PoolRecord {
                denom0,
                denom1,
                tick_spacing: TICK_SPACING,
                spread_factor: Decimal::from_str("0.01").unwrap().atomics().to_string(),
                exponent_at_price_one: "".to_string(),
            }],
        },
        signer.address(),
        signer,
    )
    .unwrap();
}

pub fn create_position(
    app: &OsmosisTestApp,
    signer: &SigningAccount,
    pool_id: u64,
    tokens_provided: Vec<OsmoCoin>,
) {
    let cl = ConcentratedLiquidity::new(app);
    cl.create_position(
        MsgCreatePosition {
            pool_id,
            sender: signer.address(),
            lower_tick: -108000000i64,
            upper_tick: 342000000i64,
            tokens_provided,
            token_min_amount0: "0".to_string(),
            token_min_amount1: "0".to_string(),
        },
        signer,
    )
    .unwrap();
}

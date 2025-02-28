use cosmwasm_std::{coin, Uint128};
use neutron_std::{
    shim::Any,
    types::{
        cosmos::adminmodule::adminmodule::MsgSubmitProposal,
        osmosis::tokenfactory::{v1beta1::MsgUpdateParams, Params, WhitelistedHook},
        slinky::{
            marketmap::v1::{Market, MsgCreateMarkets, ProviderConfig, Ticker},
            types::v1::CurrencyPair,
        },
    },
};
use neutron_test_tube::{
    cosmrs::proto::traits::Message, Account, Admin, Module, NeutronTestApp, SigningAccount, Slinky,
};
use test_tube::runner::app::SlinkyPrices;

pub const DEFAULT_LIQUIDITY: u128 = 1_000_000u128;
pub const PROPOSAL_DURATION: u64 = 1000;
pub const STRATEGY_CAP: Uint128 = Uint128::new(10_000_000_000_000u128); // 10,000 @6dp
pub const TICK_SPACING: u64 = 100;
pub const ONE_WEEK_IN_SECONDS: u64 = 604800;
pub const SPREAD_FACTOR: &str = "3000";
pub const GAS_DENOM: &str = "untrn";
pub const BASE_DENOM: &str = "ubase";
pub const STAKE_DENOM: &str = "stbase";
pub const QUOTE_DENOM: &str = "usdc";
pub const REWARD_DENOM: &str = "ureward";

pub struct TestEnv {
    pub app: NeutronTestApp,
    pub signer: SigningAccount,
    pub controller: SigningAccount,
    pub fund: SigningAccount,
    pub treasury: SigningAccount,
    pub traders: Vec<SigningAccount>,
}

impl TestEnv {
    pub fn new() -> Self {
        let app = NeutronTestApp::new();
        let slinky = Slinky::new(&app);

        let val = app
            .get_first_validator_signing_account("untrn".to_string(), 1.3)
            .unwrap();

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

        let fund = app
            .init_account(&[
                coin(1_000_000_000_000_000_000, GAS_DENOM),
                coin(1_000_000_000_000_000_000, BASE_DENOM),
                coin(1_000_000_000_000_000_000, QUOTE_DENOM),
            ])
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

        slinky
            .create_markets(
                MsgCreateMarkets {
                    authority: val.address(),
                    create_markets: vec![Market {
                        ticker: Some(Ticker {
                            currency_pair: Some(CurrencyPair {
                                base: BASE_DENOM.to_ascii_uppercase(),
                                quote: QUOTE_DENOM.to_ascii_uppercase(),
                            }),
                            decimals: 6,
                            min_provider_count: 1,
                            enabled: true,
                            metadata_json: "".to_string(),
                        }),
                        provider_configs: vec![ProviderConfig {
                            name: "margined".to_string(),
                            off_chain_ticker: "NRTN/USD".to_string(),
                            normalize_by_pair: None,
                            invert: false,
                            metadata_json: "".to_string(),
                        }],
                    }],
                },
                &val,
            )
            .unwrap();

        app.set_slinky_prices(&[SlinkyPrices {
            base: BASE_DENOM.to_ascii_uppercase(),
            quote: QUOTE_DENOM.to_ascii_uppercase(),
            price: 1250000u128, // $1.25
        }]);

        Self {
            app,
            signer,
            controller,
            fund,
            treasury,
            traders,
        }
    }

    pub fn set_slinky(&self) {
        self.app.set_slinky_prices(&[SlinkyPrices {
            base: BASE_DENOM.to_ascii_uppercase(),
            quote: QUOTE_DENOM.to_ascii_uppercase(),
            price: 1250000u128, // $1.25
        }]);
    }

    pub fn whitelist_hooks(&self, whitelisted_hooks: Vec<WhitelistedHook>) {
        let adminmodule = Admin::new(&self.app);

        let val = self
            .app
            .get_first_validator_signing_account("untrn".to_string(), 1.3)
            .unwrap();

        // address of admin moudule. it is an authority for all modules
        let adminmodule_addr = "neutron1hxskfdxpp5hqgtjj6am6nkjefhfzj359x0ar3z";

        // tokenfactory update params messaage
        let tfmsg = MsgUpdateParams {
            authority: adminmodule_addr.to_string(),
            params: Some(Params {
                // set proper params & hooks below
                denom_creation_fee: vec![],
                denom_creation_gas_consume: 0,
                fee_collector_address: "".to_string(),
                whitelisted_hooks,
            }),
        };

        // encode it to Any
        let tfmsg_any = Any {
            type_url: MsgUpdateParams::TYPE_URL.to_string(),
            value: tfmsg.encode_to_vec(),
        };

        // submit as a proposal
        let msg = MsgSubmitProposal {
            messages: vec![tfmsg_any],
            proposer: val.address(),
        };

        adminmodule.submit_proposal(msg, &val).unwrap();
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}

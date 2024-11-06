use cosmwasm_std::{coin, Decimal, Uint128};
use neutron_std::types::cosmos::{
    base::v1beta1::Coin as OsmoCoin,
    params::v1beta1::{ParamChange, ParameterChangeProposal},
};
use neutron_test_tube::{
    cosmrs::proto::traits::Message, Account, GovWithAppAccess, Module, NeutronTestApp,
    SigningAccount, Slinky,
};
use std::str::FromStr;

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

pub struct NeutronTestEnv {
    pub app: NeutronTestApp,
    pub signer: SigningAccount,
    pub controller: SigningAccount,
    pub treasury: SigningAccount,
    pub traders: Vec<SigningAccount>,
}

impl NeutronTestEnv {
    pub fn new() -> Self {
        let app = NeutronTestApp::new();
        let slinky = Slinky::new(&app);
        // let gov = GovWithAppAccess::new(&app);

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

        // // update the parameters so we can have no gas paying token as base
        // gov.propose_and_execute(
        //     "/cosmos.params.v1beta1.ParameterChangeProposal".to_string(),
        //     ParameterChangeProposal {
        //         title: "Update authorized quote denoms".to_string(),
        //         description: "Add authorized quote denoms".to_string(),
        //         changes: vec![ParamChange {
        //             subspace: "poolmanager".to_string(),
        //             key: "AuthorizedQuoteDenoms".to_string(),
        //             value: format!(
        //                 "[\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"]",
        //                 GAS_DENOM, BASE_DENOM, QUOTE_DENOM, REWARD_DENOM, STAKE_DENOM
        //             ),
        //         }],
        //     },
        //     signer.address(),
        //     &signer,
        // )
        // .unwrap();

        Self {
            app,
            signer,
            controller,
            treasury,
            traders,
        }
    }
}

impl Default for NeutronTestEnv {
    fn default() -> Self {
        Self::new()
    }
}

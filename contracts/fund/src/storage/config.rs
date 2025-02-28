use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure, Decimal, DepsMut, Env, Event, MessageInfo, Response, StdError, Uint128,
};
use cw_storage_plus::Item;
use interface::fund::{InstantiateMsg, UpdateConfig};
use serde::{de::DeserializeOwned, Serialize};
use vaultenator::{config::Configure, errors::ContractError, state::OWNER};

#[cw_serde]
pub struct V003Config {
    pub controller: String,
    pub admin: String,
    pub treasury: String,
    pub strategy_cap: Uint128,
    pub float: Decimal,
    pub strategy_denom: String,
    pub token0: String,
    pub token1: Option<String>,
    pub performance_fee_rate: Decimal,
    pub estimate_cycle_profit: Option<Decimal>,
    pub vault_type: String,
}

#[cw_serde]
pub struct Config {
    pub controller: String,
    pub admin: String,
    pub treasury: String,
    pub redemption_contract: String,
    pub strategy_cap: Uint128,
    pub float: Option<Decimal>,
    pub strategy_denom: String,
    pub token0: String,
    pub token1: Option<String>,
    pub management_fee_rate: Decimal,
    pub performance_fee_rate: Decimal,
    pub estimate_cycle_profit: Option<Decimal>,
    pub vault_type: String,
}

impl Configure for Config {
    const CONFIG_KEY: &'static str = "config";

    fn update_strategy_denom(&mut self, denom: String) {
        self.strategy_denom = denom;
    }

    fn init_config<M>(deps: &mut DepsMut, msg: &M) -> Result<Self, ContractError>
    where
        M: Serialize + DeserializeOwned,
    {
        // Deserialize the message directly into an InstantiateMsg struct
        let msg: InstantiateMsg = serde_json::from_slice(&serde_json::to_vec(msg).unwrap())?;

        let config = Self {
            admin: deps.api.addr_validate(&msg.admin)?.to_string(),
            controller: deps.api.addr_validate(&msg.controller)?.to_string(),
            treasury: deps.api.addr_validate(&msg.treasury)?.to_string(),
            redemption_contract: msg.redemption_contract,
            strategy_cap: msg.strategy_cap,
            float: msg.float,
            strategy_denom: "".to_string(),
            token0: msg.token0,
            token1: msg.token1,
            management_fee_rate: msg.management_fee_rate,
            performance_fee_rate: msg.performance_fee_rate,
            estimate_cycle_profit: None,
            vault_type: msg.vault_type,
        };

        config.validate(deps)?;
        config.save_to_storage(deps)?;

        Ok(config)
    }

    fn handle_update_config<M>(
        deps: &mut DepsMut,
        info: MessageInfo,
        _env: Env,
        msg: M,
    ) -> Result<Response, ContractError>
    where
        M: Serialize + DeserializeOwned,
    {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let mut config = Self::get_from_storage(deps.as_ref())?;
        let update_msg: UpdateConfig =
            serde_json::from_slice(&serde_json::to_vec(&msg)?).map_err(|e| {
                ContractError::Std(StdError::generic_err(format!(
                    "Deserialization error: {}",
                    e
                )))
            })?;

        // Update validated addresses
        if let Some(controller) = update_msg.controller {
            config.controller = deps.api.addr_validate(&controller)?.to_string();
        }
        if let Some(treasury) = update_msg.treasury {
            config.treasury = deps.api.addr_validate(&treasury)?.to_string();
        }
        if let Some(redemption_contract) = update_msg.redemption_contract {
            deps.api.addr_validate(&redemption_contract)?;

            config.redemption_contract = redemption_contract;
        }

        // Update numeric values
        if let Some(strategy_cap) = update_msg.strategy_cap {
            config.strategy_cap = strategy_cap;
        }
        if let Some(management_fee_rate) = update_msg.management_fee_rate {
            config.management_fee_rate = management_fee_rate;
        }
        if let Some(performance_fee_rate) = update_msg.performance_fee_rate {
            config.performance_fee_rate = performance_fee_rate;
        }
        if let Some(estimate_cycle_profit) = update_msg.estimate_cycle_profit {
            config.estimate_cycle_profit = Some(estimate_cycle_profit);
        }
        if let Some(float) = update_msg.float {
            config.float = float;
        }

        config.validate(deps)?;
        config.save_to_storage(deps)?;

        Ok(Response::new().add_event(Event::new("update_config")))
    }

    fn validate(&self, _deps: &mut DepsMut) -> Result<(), ContractError> {
        if self.strategy_denom == self.token0 {
            return Err(ContractError::Std(StdError::generic_err(
                "Invalid configuration: strategy_denom and token0 must be different",
            )));
        }

        if let Some(denom) = &self.token1 {
            if self.strategy_denom == *denom {
                return Err(ContractError::Std(StdError::generic_err(
                    "Invalid configuration: strategy_denom and token1 must be different",
                )));
            }
        }

        ensure!(
            self.strategy_cap > Uint128::zero(),
            ContractError::Std(StdError::generic_err(
                "Strategy cap must be greater than zero"
            ))
        );

        ensure!(
            self.performance_fee_rate <= Decimal::percent(20),
            ContractError::Std(StdError::generic_err(
                "Performance fee rate must be less or equal to twenty percent"
            ))
        );

        ensure!(
            self.float.unwrap_or(Decimal::zero()) <= Decimal::percent(10),
            ContractError::Std(StdError::generic_err(
                "Float must be less or equal to ten percent"
            ))
        );

        ensure!(
            self.management_fee_rate <= Decimal::percent(5),
            ContractError::Std(StdError::generic_err(
                "Management fee must be less or equal to five percent"
            ))
        );

        Ok(())
    }
}

impl Config {
    pub fn get_denoms(&self) -> Vec<String> {
        if let Some(token1) = &self.token1 {
            vec![self.token0.clone(), token1.to_string()]
        } else {
            vec![self.token0.clone()]
        }
    }
}

pub fn migrate_config(
    mut deps: DepsMut,
    redemption_contract: String,
) -> Result<(DepsMut, Response), ContractError> {
    let old_config: Item<V003Config> = Item::new("config");

    let cfg = old_config.load(deps.storage)?;

    let new_config = Config {
        admin: cfg.admin,
        controller: cfg.controller,
        treasury: cfg.treasury,
        redemption_contract,
        strategy_cap: cfg.strategy_cap,
        float: Some(cfg.float),
        strategy_denom: cfg.strategy_denom,
        token0: cfg.token0,
        token1: cfg.token1,
        management_fee_rate: Decimal::zero(),
        performance_fee_rate: cfg.performance_fee_rate,
        estimate_cycle_profit: cfg.estimate_cycle_profit,
        vault_type: cfg.vault_type,
    };

    new_config.validate(&mut deps)?;
    new_config.save_to_storage(&mut deps)?;

    Ok((
        deps,
        Response::new().add_event(Event::new("migrate_config")),
    ))
}

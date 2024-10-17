use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure, Decimal, DepsMut, Env, Event, MessageInfo, Response, StdError, Uint128,
};
use cw_storage_plus::Item;
use interface::fund::{InstantiateMsg, UpdateConfig};
use serde::{de::DeserializeOwned, Serialize};
use vaultenator::{config::Configure, errors::ContractError, state::OWNER};

#[cw_serde]
pub struct Config {
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
            strategy_cap: msg.strategy_cap,
            float: msg.float,
            strategy_denom: "".to_string(),
            token0: msg.token0,
            token1: msg.token1,
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
        // Function can only be called by admin
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        // Load the current configuration from storage
        let mut config = Self::get_from_storage(deps.as_ref())?;

        // // Deserialize the message directly into an UpdateConfig struct
        let update_msg: UpdateConfig =
            serde_json::from_slice(&serde_json::to_vec(&msg)?).map_err(|e| {
                ContractError::Std(StdError::generic_err(format!(
                    "Deserialization error: {}",
                    e
                )))
            })?;

        // Initialise a mutable response
        let response = Response::new().add_event(Event::new("update_config"));

        // Update config with new values
        if let Some(new_strategy_cap) = update_msg.strategy_cap {
            config.strategy_cap = new_strategy_cap;
        }

        if let Some(new_controller) = update_msg.controller {
            config.controller = deps.api.addr_validate(&new_controller)?.to_string();
        }

        if let Some(performance_fee_rate) = update_msg.performance_fee_rate {
            config.performance_fee_rate = performance_fee_rate;
        }

        if let Some(treasury) = update_msg.treasury {
            config.treasury = deps.api.addr_validate(&treasury)?.to_string();
        }

        if let Some(float) = update_msg.float {
            config.float = float;
        }

        config.validate(deps)?;
        config.save_to_storage(deps)?;

        Ok(response)
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
            self.float <= Decimal::percent(10),
            ContractError::Std(StdError::generic_err(
                "Float must be less or equal to ten percent"
            ))
        );

        Ok(())
    }
}

#[cw_serde]
pub struct OldConfig {
    pub controller: String,
    pub admin: String,
    pub treasury: String,
    pub strategy_cap: Uint128,
    pub float: Uint128,
    pub strategy_denom: String,
    pub token0: String,
    pub token1: Option<String>,
    pub performance_fee_rate: Decimal,
    pub estimate_cycle_profit: Option<Decimal>,
    pub vault_type: String,
}

pub fn migrate_config(mut deps: DepsMut) -> Result<Response, ContractError> {
    let old_config: Item<OldConfig> = Item::new("config");

    let cfg = old_config.load(deps.storage)?;

    let new_config = Config {
        controller: cfg.controller,
        admin: cfg.admin,
        treasury: cfg.treasury,
        strategy_cap: cfg.strategy_cap,
        float: Decimal::zero(),
        strategy_denom: cfg.strategy_denom,
        token0: cfg.token0,
        token1: cfg.token1,
        performance_fee_rate: cfg.performance_fee_rate,
        estimate_cycle_profit: cfg.estimate_cycle_profit,
        vault_type: cfg.vault_type,
    };

    new_config.validate(&mut deps)?;
    new_config.save_to_storage(&mut deps)?;

    Ok(Response::new().add_event(Event::new("migrate_config")))
}

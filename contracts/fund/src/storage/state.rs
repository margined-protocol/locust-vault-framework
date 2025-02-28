use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr, Coin, Decimal, Deps, DepsMut, Env, Event, Response, Storage, Timestamp, Uint128,
};
use cw_storage_plus::{Item, Map};
use std::collections::HashMap;
use vaultenator::{errors::ContractError, state::ManageState};

pub const TWAP_PERIOD: u64 = 420; // 7 minutes
pub const DEFAULT_QUEUE_LIMIT: u64 = 50;

pub const USER_DEPOSITS: Map<Addr, UserDeposit> = Map::new("user_deposits");

#[cw_serde]
pub struct UserDeposit {
    pub total_deposits: Uint128,
    pub timestamp: u64,
}

#[cw_serde]
pub struct V003State {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
    pub last_claim: Timestamp,
    pub total_staked_tokens: Uint128,
    pub total_withdrawn_tokens: HashMap<String, Uint128>,
}

#[cw_serde]
pub struct State {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
    pub last_claim: Timestamp,
    pub pending_management_fees: Vec<Coin>,
    pub total_staked_tokens: Uint128,
    pub total_withdrawn_tokens: HashMap<String, Uint128>,
}

impl ManageState for State {
    const STATE_KEY: &'static str = "state";

    fn is_contract_open(deps: Deps) -> Result<bool, ContractError> {
        let state = Self::get_from_storage(deps)?;
        Ok(state.is_open)
    }

    fn is_contract_paused(deps: Deps) -> Result<bool, ContractError> {
        let state = Self::get_from_storage(deps)?;
        Ok(state.is_paused)
    }

    fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    fn init_state(deps: &mut DepsMut, env: &Env) -> Result<(), ContractError> {
        let initial_state = State {
            is_open: false,
            is_paused: false,
            last_pause: env.block.time,
            last_claim: env.block.time,
            pending_management_fees: vec![],
            total_staked_tokens: Uint128::zero(),
            total_withdrawn_tokens: HashMap::new(),
        };

        initial_state.save_to_storage(deps)
    }

    fn update_state(&mut self, deps: &mut DepsMut) -> Result<(), ContractError> {
        self.is_paused = !self.is_paused;
        self.save_to_storage(deps)
    }
}

impl State {
    pub fn add_to_total_staked_tokens(&mut self, amount: Uint128) -> Result<(), ContractError> {
        self.total_staked_tokens = self
            .total_staked_tokens
            .checked_add(amount)
            .map_err(ContractError::Overflow)?;
        Ok(())
    }

    pub fn remove_from_total_staked_tokens(
        &mut self,
        amount: Uint128,
    ) -> Result<(), ContractError> {
        self.total_staked_tokens = self
            .total_staked_tokens
            .checked_sub(amount)
            .map_err(ContractError::Overflow)?;
        Ok(())
    }

    pub fn add_to_total_withdrawn_tokens(
        &mut self,
        amount: Uint128,
        denom: &str,
    ) -> Result<(), ContractError> {
        let entry = self
            .total_withdrawn_tokens
            .entry(denom.to_string())
            .or_insert(Uint128::zero());
        *entry += amount;

        Ok(())
    }

    pub fn get_total_withdrawn_tokens(&self, denom: &str) -> Uint128 {
        self.total_withdrawn_tokens
            .get(denom)
            .copied()
            .unwrap_or(Uint128::zero())
    }

    pub fn remove_from_total_withdrawn_tokens(
        &mut self,
        amount: Uint128,
        denom: &str,
    ) -> Result<(), ContractError> {
        let entry = self
            .total_withdrawn_tokens
            .entry(denom.to_string())
            .or_insert(Uint128::zero());

        *entry = entry.saturating_sub(amount);

        Ok(())
    }

    pub fn update_last_claim(&mut self, latest_timestamp: Timestamp) -> Result<(), ContractError> {
        self.last_claim = latest_timestamp;

        Ok(())
    }

    pub fn update_pending_management_fees(&mut self, fees: Vec<Coin>) -> Result<(), ContractError> {
        self.pending_management_fees = fees;

        Ok(())
    }
}

pub fn migrate_state(mut deps: DepsMut) -> Result<Response, ContractError> {
    let state: Item<V003State> = Item::new("state");

    let stt = state.load(deps.storage)?;

    let new_state = State {
        is_open: stt.is_open,
        is_paused: stt.is_paused,
        last_pause: stt.last_pause,
        last_claim: stt.last_claim,
        pending_management_fees: vec![],
        total_staked_tokens: stt.total_staked_tokens,
        total_withdrawn_tokens: stt.total_withdrawn_tokens,
    };

    new_state.save_to_storage(&mut deps)?;

    Ok(Response::new().add_event(Event::new("migrate_state")))
}

impl UserDeposit {
    pub fn empty_deposit(timestamp: u64) -> Self {
        Self {
            total_deposits: Uint128::zero(),
            timestamp,
        }
    }

    pub fn load_or_initialize_user_deposit(
        storage: &dyn Storage,
        sender: &Addr,
        current_time: u64,
    ) -> Result<UserDeposit, ContractError> {
        Ok(USER_DEPOSITS
            .may_load(storage, sender.clone())?
            .unwrap_or(UserDeposit::empty_deposit(current_time)))
    }

    pub fn add_to_user_deposits(&mut self, amount: Uint128) -> Result<(), ContractError> {
        self.total_deposits = self
            .total_deposits
            .checked_add(amount)
            .map_err(ContractError::Overflow)?;
        Ok(())
    }

    pub fn remove_from_user_deposits(&mut self, amount: Uint128) -> Result<(), ContractError> {
        self.total_deposits = self
            .total_deposits
            .checked_sub(amount)
            .map_err(ContractError::Overflow)?;
        Ok(())
    }
}

pub fn update_user_deposit(
    storage: &mut dyn Storage,
    sender: Addr,
    burn_ratio: Decimal,
    state: &mut State,
) -> Result<(), ContractError> {
    let mut user_deposit = USER_DEPOSITS.load(storage, sender.clone())?;
    let user_deposit_redeemed = user_deposit.total_deposits.mul_floor(burn_ratio);

    user_deposit.remove_from_user_deposits(user_deposit_redeemed)?;
    state.remove_from_total_staked_tokens(user_deposit_redeemed)?;

    USER_DEPOSITS.save(storage, sender, &user_deposit)?;

    Ok(())
}

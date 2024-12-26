use crate::{
    contract::{StructuredVault, CONTRACT_NAME, CONTRACT_VERSION},
    events::{event_deposit, event_migrate},
    handlers::extensions::get_assets_to_burn,
    helpers::{
        calculate_assets_to_redeem, calculate_assets_value, check_strategy_cap, get_amount_to_mint,
        get_deposit_value, get_sent_tokens, get_strategy_denom, get_token_deposits,
        map_to_contract_error,
    },
    process::{process_deposit, process_management_fees_and_modify_response, process_redeem},
    queries::{get_balance, get_total_supply},
    reply::ReplyIDs,
    storage::{
        config::{migrate_config, Config},
        state::{migrate_state, update_user_deposit, State, UserDeposit, USER_DEPOSITS},
    },
};

use cosmwasm_std::{coin, Decimal, DepsMut, Env, MessageInfo, Response, StdError, SubMsg, Uint128};
use cw2::{get_contract_version, set_contract_version};
use cw_utils::{must_pay, nonpayable};
use serde::{de::DeserializeOwned, Serialize};
use vaultenator::{
    config::Configure,
    contract::Describe,
    errors::ContractError,
    handlers::Handle,
    msg::create_denom_message,
    state::{ManageState, OWNER},
};

impl Handle<Config, State> for StructuredVault {
    fn handle_instantiate<M>(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: M,
    ) -> Result<Response, ContractError>
    where
        M: Serialize + DeserializeOwned,
    {
        Config::init_config(&mut deps, &msg)?;
        State::init_state(&mut deps, &env)?;

        let mut config = Config::get_from_storage(deps.as_ref())?;

        set_contract_version(
            deps.storage,
            format!("crates.io:{}", Self::CONTRACT_NAME),
            env!("CARGO_PKG_VERSION"),
        )?;

        let deposit =
            must_pay(&info, &config.token0).map_err(|_| ContractError::InvalidFunds {})?;

        let create_denom_sub_msg = SubMsg::reply_always(
            create_denom_message(&env.contract.address, Self::CONTRACT_NAME.to_string()),
            ReplyIDs::CreateStrategyDenom as u64,
        );

        config.update_strategy_denom(get_strategy_denom(&env, CONTRACT_NAME));

        config.save_to_storage(&mut deps)?;

        OWNER.set(deps, Some(info.sender.clone()))?;

        Ok(Response::new()
            .add_submessage(create_denom_sub_msg)
            .add_attribute("action", "instantiate")
            .add_event(event_deposit(
                CONTRACT_VERSION,
                CONTRACT_NAME,
                env.contract.address.as_ref(),
                coin(deposit.into(), config.token0.clone()),
                None,
            )))
    }

    fn handle_deposit(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        _amount: Uint128, // not used when sending funds
        _recipient: Option<String>,
    ) -> Result<Response, ContractError> {
        // Ensure the contract is open and unpaused
        State::is_open_and_unpaused(deps.as_ref())?;

        let config = Config::get_from_storage(deps.as_ref())?;
        let sent_tokens = get_sent_tokens(&info, &config)?;
        let deposit_value = get_deposit_value(&deps.as_ref(), &config, sent_tokens.clone())?;

        // Process management fees and modify the response accordingly
        let (mut response, mut deps) = process_management_fees_and_modify_response(
            deps,
            Response::default(),
            env.clone(),
            Some(sent_tokens.clone()),
        )?;

        // Update contract state with new deposit
        let mut state = State::get_from_storage(deps.as_ref())?;

        state.add_to_total_staked_tokens(deposit_value)?;
        check_strategy_cap(&config, &state)?;

        state.save_to_storage(&mut deps)?;

        let (token0, token1) = get_token_deposits(&config, sent_tokens)?;

        // Update user's deposit record
        let mut user_deposit = UserDeposit::load_or_initialize_user_deposit(
            deps.storage,
            &info.sender,
            env.block.time.seconds(),
        )?;
        user_deposit.add_to_user_deposits(deposit_value)?;

        USER_DEPOSITS.save(deps.storage, info.sender.clone(), &user_deposit)?;

        let current_assets = calculate_assets_value(
            &deps.as_ref(),
            &config,
            &state,
            env.contract.address.as_str(),
        )?;

        let previous_assets = current_assets
            .checked_sub(deposit_value)
            .map_err(ContractError::Overflow)?;

        let amount_to_mint = get_amount_to_mint(
            &deps.as_ref(),
            &current_assets,
            &previous_assets,
            &config.strategy_denom,
        )?;

        response = process_deposit(
            response,
            amount_to_mint,
            &info.sender,
            &env.contract.address,
            &config,
        )?;

        Ok(response.add_event(event_deposit(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            info.sender.as_ref(),
            token0,
            token1,
        )))
    }

    fn handle_redeem(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        _amount: Uint128, // not used when sending funds
        _recipient: Option<String>,
    ) -> Result<Response, ContractError> {
        let (mut response, mut deps) = process_management_fees_and_modify_response(
            deps,
            Response::default(),
            env.clone(),
            None,
        )?;

        State::is_open_and_unpaused(deps.as_ref())?;

        let config = Config::get_from_storage(deps.as_ref())?;
        let mut state = State::get_from_storage(deps.as_ref())?;

        let strategy_denom_sent =
            must_pay(&info, &config.strategy_denom).map_err(|_| ContractError::InvalidFunds {})?;

        let (burn_ratio, assets_to_redeem) = get_assets_to_burn(
            deps.as_ref(),
            &config,
            &state,
            info.sender.as_str(),
            env.contract.address.as_str(),
            strategy_denom_sent,
        )?;

        update_user_deposit(deps.storage, info.sender.clone(), burn_ratio, &mut state)?;

        state.save_to_storage(&mut deps)?;

        response = process_redeem(
            response,
            &info,
            assets_to_redeem,
            &config,
            &env,
            strategy_denom_sent,
        )?;

        Ok(response)
    }

    fn handle_migrate<M>(
        &self,
        deps: DepsMut,
        _env: Env,
        _msg: M,
    ) -> Result<Response, ContractError>
    where
        M: Serialize + DeserializeOwned,
    {
        let contract_version = get_contract_version(deps.storage)?;

        match contract_version.contract.as_ref() {
            "crates.io:fund" => match contract_version.version.as_ref() {
                "0.0.4" => {
                    set_contract_version(
                        deps.storage,
                        format!("crates.io:{CONTRACT_NAME}"),
                        CONTRACT_VERSION,
                    )?;

                    let (deps, _) = migrate_config(deps)?;
                    migrate_state(deps)?;
                }
                "0.0.5" => {
                    set_contract_version(
                        deps.storage,
                        format!("crates.io:{CONTRACT_NAME}"),
                        CONTRACT_VERSION,
                    )?;
                }
                _ => {
                    return Err(ContractError::Std(StdError::generic_err(
                        "Migration failed",
                    )))
                }
            },
            _ => {
                return Err(ContractError::Std(StdError::generic_err(
                    "Migration failed",
                )))
            }
        }

        Ok(Response::new().add_event(event_migrate(
            CONTRACT_VERSION,
            CONTRACT_NAME,
            contract_version,
        )))
    }

    fn handle_crank(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        nonpayable(&info).map_err(map_to_contract_error)?;

        let (response, _) =
            process_management_fees_and_modify_response(deps, Response::default(), env, None)?;

        Ok(response)
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Order, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex};

#[cw_serde]
pub struct Redemption {
    pub user: Addr,
    pub total_deposits: Uint128,
    pub timestamp: u64,
}

pub struct Queue<'a> {
    pub owner: MultiIndex<'a, Addr, Redemption, String>,
}

impl<'a> IndexList<Redemption> for Queue<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Redemption>> + '_> {
        let v: Vec<&dyn Index<Redemption>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
pub fn redemptions<'a>() -> IndexedMap<&'a str, Redemption, Queue<'a>> {
    let indexes = Queue {
        owner: MultiIndex::new(
            |_, value| value.user.clone(),
            "redemption",
            "redemption__owner",
        ),
    };
    IndexedMap::new("redemption", indexes)
}

pub fn redemptions_by_owner<'a>() -> MultiIndex<'a, Addr, Redemption, String> {
    redemptions().idx.owner
}

pub fn filter_queue_by_user<'a>(
    storage: &'a dyn Storage,
    user: Addr,
) -> Box<dyn Iterator<Item = StdResult<(String, Redemption)>> + 'a> {
    redemptions_by_owner()
        .prefix(user)
        .range(storage, None, None, Order::Ascending)
}

pub fn get_total_size_of_queue(storage: &dyn Storage, user: Addr) -> usize {
    filter_queue_by_user(storage, user).count()
}

pub fn get_all_user_redemptions(storage: &dyn Storage, user: Addr) -> StdResult<Vec<Redemption>> {
    Ok(
        filter_queue_by_user(storage, user)
            .collect::<StdResult<Vec<_>>>()? // Collect into StdResult and propagate errors with `?`
            .iter()
            .map(|(_, strategy)| strategy.clone())
            .collect::<Vec<_>>(), // Collect the final Vec
    )
}

pub fn get_all_redemptions<'a>(
    storage: &dyn Storage,
    start_bound: Option<Bound<'a, &'a str>>,
    query_limit: usize,
) -> StdResult<Vec<Redemption>> {
    redemptions()
        .range(storage, start_bound, None, Order::Ascending)
        .take(query_limit)
        .map(|item| {
            let (_, strategy) = item?;
            Ok(strategy)
        })
        .collect()
}

pub fn add_to_queue(
    storage: &mut dyn Storage,
    user: Addr,
    amount: Uint128,
    timestamp: u64,
) -> StdResult<()> {
    let redemptions_map = redemptions();

    // Check if user already exists in the queue
    match redemptions_map.may_load(storage, user.as_str())? {
        Some(mut redemption) => {
            // Update the user's total deposits and timestamp
            redemption.total_deposits += amount;
            redemption.timestamp = timestamp;
            redemptions_map.save(storage, user.as_str(), &redemption)?;
        }
        None => {
            // Initialize a new Redemption entry for the user
            let redemption = Redemption {
                user: user.clone(),
                total_deposits: amount,
                timestamp,
            };
            redemptions_map.save(storage, user.as_str(), &redemption)?;
        }
    }

    Ok(())
}

pub fn remove_from_queue(storage: &mut dyn Storage, user: Addr) -> StdResult<Uint128> {
    let redemptions_map = redemptions();

    // Check if user exists in the queue
    match redemptions_map.may_load(storage, user.as_str())? {
        Some(redemption) => {
            let amount = redemption.total_deposits;

            redemptions_map.remove(storage, user.as_str())?;

            Ok(amount)
        }
        None => {
            // User does not exist in the queue
            Err(StdError::not_found("Redemption"))
        }
    }
}

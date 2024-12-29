use cosmwasm_std::{Order, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use interface::fund::Redemption;

pub struct Queue<'a> {
    pub timestamp: MultiIndex<'a, u64, Redemption, String>,
    pub user: MultiIndex<'a, String, Redemption, String>,
}

impl<'a> IndexList<Redemption> for Queue<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Redemption>> + '_> {
        let v: Vec<&dyn Index<Redemption>> = vec![&self.timestamp, &self.user];
        Box::new(v.into_iter())
    }
}
pub fn redemptions<'a>() -> IndexedMap<&'a str, Redemption, Queue<'a>> {
    let indexes = Queue {
        timestamp: MultiIndex::new(
            |_, value| value.timestamp,
            "redemption",
            "redemption__timestamp",
        ),
        user: MultiIndex::new(
            |_, value| value.user.clone(),
            "redemption",
            "redemption__user",
        ),
    };
    IndexedMap::new("redemption", indexes)
}

pub fn redemptions_by_user<'a>() -> MultiIndex<'a, String, Redemption, String> {
    redemptions().idx.user
}

pub fn redemptions_by_timestamp<'a>() -> MultiIndex<'a, u64, Redemption, String> {
    redemptions().idx.timestamp
}

pub fn filter_queue_by_user<'a>(
    storage: &'a dyn Storage,
    user: String,
) -> Box<dyn Iterator<Item = StdResult<(String, Redemption)>> + 'a> {
    redemptions_by_user()
        .prefix(user)
        .range(storage, None, None, Order::Ascending)
}

pub fn get_total_size_of_queue(storage: &dyn Storage, user: String) -> usize {
    filter_queue_by_user(storage, user).count()
}

pub fn get_all_user_redemptions(storage: &dyn Storage, user: String) -> StdResult<Vec<Redemption>> {
    Ok(
        filter_queue_by_user(storage, user)
            .collect::<StdResult<Vec<_>>>()? // Collect into StdResult and propagate errors with `?`
            .iter()
            .map(|(_, strategy)| strategy.clone())
            .collect::<Vec<_>>(), // Collect the final Vec
    )
}

pub fn get_all_redemptions(storage: &dyn Storage, limit: usize) -> StdResult<Vec<Redemption>> {
    redemptions()
        .range(storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, strategy) = item?;
            Ok(strategy)
        })
        .collect()
}

pub fn iterate_redemptions_by_timestamp<'a>(
    storage: &'a dyn Storage,
    limit: Option<usize>,
) -> Box<dyn Iterator<Item = StdResult<(String, Redemption)>> + 'a> {
    Box::new(
        redemptions()
            .idx
            .timestamp
            .range(storage, None, None, Order::Ascending)
            .take(limit.unwrap_or(usize::MAX)), // Apply the limit, or use usize::MAX if None
    )
}
pub fn add_to_queue(
    storage: &mut dyn Storage,
    user: String,
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

pub fn remove_from_queue(storage: &mut dyn Storage, user: String) -> StdResult<Uint128> {
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

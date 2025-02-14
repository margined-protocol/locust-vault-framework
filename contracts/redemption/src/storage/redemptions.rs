use cosmwasm_std::{Addr, Order, StdError, StdResult, Storage, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use interface::redemption::PendingRedemption;

pub struct Pending<'a> {
    pub user: MultiIndex<'a, Addr, PendingRedemption, String>,
}

impl<'a> IndexList<PendingRedemption> for Pending<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<PendingRedemption>> + '_> {
        let v: Vec<&dyn Index<PendingRedemption>> = vec![&self.user];
        Box::new(v.into_iter())
    }
}
pub fn redemptions<'a>() -> IndexedMap<&'a str, PendingRedemption, Pending<'a>> {
    let indexes = Pending {
        user: MultiIndex::new(
            |_, value| Addr::unchecked(value.user.clone()),
            "redemption",
            "redemption__user",
        ),
    };
    IndexedMap::new("redemption", indexes)
}

pub fn redemptions_by_user<'a>() -> MultiIndex<'a, Addr, PendingRedemption, String> {
    redemptions().idx.user
}

pub fn filter_user_redemptions<'a>(
    storage: &'a dyn Storage,
    user: String,
) -> Box<dyn Iterator<Item = StdResult<(String, PendingRedemption)>> + 'a> {
    redemptions_by_user()
        .prefix(Addr::unchecked(user))
        .range(storage, None, None, Order::Ascending)
}

pub fn get_total_size_of_queue(storage: &dyn Storage, user: String) -> usize {
    filter_user_redemptions(storage, user).count()
}

pub fn get_all_user_redemptions(
    storage: &dyn Storage,
    user: String,
) -> StdResult<Vec<PendingRedemption>> {
    Ok(
        filter_user_redemptions(storage, user)
            .collect::<StdResult<Vec<_>>>()? // Collect into StdResult and propagate errors with `?`
            .iter()
            .map(|(_, strategy)| strategy.clone())
            .collect::<Vec<_>>(), // Collect the final Vec
    )
}

pub fn add_to_pending(storage: &mut dyn Storage, redemption: PendingRedemption) -> StdResult<()> {
    let redemptions_map = redemptions();

    // Check if user already exists in the queue
    match redemptions_map.may_load(storage, &redemption.user)? {
        Some(_) => {
            // continue
        }
        None => {
            redemptions_map.save(storage, redemption.user.as_str(), &redemption)?;
        }
    }

    Ok(())
}

// pub fn remove_from_pending(storage: &mut dyn Storage, user: String) -> StdResult<Uint128> {
//     let redemptions_map = redemptions();

//     // Check if user exists in the queue
//     match redemptions_map.may_load(storage, user.as_str())? {
//         Some(redemption) => {
//             let amount = redemption.total_deposits;

//             redemptions_map.remove(storage, user.as_str())?;

//             Ok(amount)
//         }
//         None => {
//             // User does not exist in the queue
//             Err(StdError::not_found("Redemption"))
//         }
//     }
// }

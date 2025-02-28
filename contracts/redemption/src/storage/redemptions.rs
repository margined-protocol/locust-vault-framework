use cosmwasm_std::{Addr, Order, StdError, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use interface::redemption::PendingRedemption;

type RedemptionResult<'a> =
    Box<dyn Iterator<Item = StdResult<((String, u64), PendingRedemption)>> + 'a>;

pub struct Pending<'a> {
    // Index by user address
    pub user: MultiIndex<'a, Addr, PendingRedemption, (String, u64)>,
}

impl<'a> IndexList<PendingRedemption> for Pending<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<PendingRedemption>> + '_> {
        let v: Vec<&dyn Index<PendingRedemption>> = vec![&self.user];
        Box::new(v.into_iter())
    }
}

// Create indexed map with composite key (user, timestamp)
pub fn redemptions<'a>() -> IndexedMap<(&'a str, u64), PendingRedemption, Pending<'a>> {
    let indexes = Pending {
        user: MultiIndex::new(
            |_, value| Addr::unchecked(value.user.clone()),
            "redemption",
            "redemption__user",
        ),
    };
    IndexedMap::new("redemption", indexes)
}

pub fn redemptions_by_user<'a>() -> MultiIndex<'a, Addr, PendingRedemption, (String, u64)> {
    redemptions().idx.user
}

pub fn filter_user_redemptions(storage: &'_ dyn Storage, user: String) -> RedemptionResult<'_> {
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
    Ok(filter_user_redemptions(storage, user)
        .collect::<StdResult<Vec<_>>>()?
        .iter()
        .map(|(_, redemption)| redemption.clone())
        .collect::<Vec<_>>())
}

pub fn add_to_pending(storage: &mut dyn Storage, redemption: PendingRedemption) -> StdResult<()> {
    let redemptions_map = redemptions();
    let key = (redemption.user.as_str(), redemption.timestamp);

    // Save the redemption with composite key
    redemptions_map.save(storage, key, &redemption)?;

    Ok(())
}

pub fn remove_from_pending(
    storage: &mut dyn Storage,
    user: String,
    timestamp: u64,
) -> StdResult<PendingRedemption> {
    let redemptions_map = redemptions();
    let key = (user.as_str(), timestamp);

    // Check if redemption exists
    match redemptions_map.may_load(storage, key)? {
        Some(redemption) => {
            redemptions_map.remove(storage, key)?;
            Ok(redemption)
        }
        None => Err(StdError::not_found("Redemption not found")),
    }
}

use crate::storage::queue::{add_to_queue, redemptions, remove_from_queue};

use cosmwasm_std::{testing::MockStorage, Addr, Uint128};

#[test]
fn test_add_to_queue() {
    let mut storage = MockStorage::new();
    let user = Addr::unchecked("user1");

    // Add the first entry
    add_to_queue(
        &mut storage,
        user.to_string(),
        Uint128::new(100),
        1672531200,
    )
    .unwrap();
    let redemption = redemptions().load(&storage, user.as_str()).unwrap();
    assert_eq!(redemption.total_deposits, Uint128::new(100));
    assert_eq!(redemption.timestamp, 1672531200);

    // Add more to the existing entry
    add_to_queue(&mut storage, user.to_string(), Uint128::new(50), 1672531300).unwrap();
    let redemption = redemptions().load(&storage, user.as_str()).unwrap();
    assert_eq!(redemption.total_deposits, Uint128::new(150));
    assert_eq!(redemption.timestamp, 1672531300); // Ensure timestamp is updated
}

#[test]
fn test_remove_from_queue() {
    let mut storage = MockStorage::new();
    let user = Addr::unchecked("user1");

    // Add initial entry
    add_to_queue(
        &mut storage,
        user.to_string(),
        Uint128::new(100),
        1672531200,
    )
    .unwrap();

    // Remove a partial amount
    let amount = remove_from_queue(&mut storage, user.to_string()).unwrap();
    redemptions().load(&storage, user.as_str()).unwrap_err();

    assert_eq!(amount, Uint128::new(100));
}

#[test]
fn test_remove_nonexistent_user() {
    let mut storage = MockStorage::new();
    let user = Addr::unchecked("user1");

    // Attempt to remove a non-existent user
    let result = remove_from_queue(&mut storage, user.to_string());
    assert!(result.is_err()); // Expect an error
}

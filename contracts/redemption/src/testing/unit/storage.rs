use crate::storage::redemptions::{
    add_to_pending, get_all_user_redemptions, get_total_size_of_queue, redemptions,
    remove_from_pending,
};

use cosmwasm_std::testing::MockStorage;
use cosmwasm_std::{coins, StdError};
use interface::redemption::PendingRedemption;

fn create_test_redemption(user: &str, timestamp: u64) -> PendingRedemption {
    PendingRedemption {
        user: user.to_string(),
        funds: coins(100, "uatom"),
        timestamp,
        source: "test_source".to_string(),
    }
}

#[test]
fn test_add_and_remove_redemption() {
    let mut storage = MockStorage::new();
    let redemption = create_test_redemption("user1", 1000);

    // Test adding redemption
    add_to_pending(&mut storage, redemption.clone()).unwrap();

    // Verify redemption was added
    let loaded = redemptions()
        .load(&storage, (redemption.user.as_str(), redemption.timestamp))
        .unwrap();
    assert_eq!(loaded, redemption);

    // Test removing redemption
    let removed = remove_from_pending(&mut storage, "user1".to_string(), 1000).unwrap();
    assert_eq!(removed, redemption);

    // Verify redemption was removed
    let result = redemptions().may_load(&storage, ("user1", 1000)).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_multiple_redemptions_per_user() {
    let mut storage = MockStorage::new();

    // Add multiple redemptions for the same user
    let redemption1 = create_test_redemption("user1", 1000);
    let redemption2 = create_test_redemption("user1", 2000);
    let redemption3 = create_test_redemption("user1", 3000);

    add_to_pending(&mut storage, redemption1.clone()).unwrap();
    add_to_pending(&mut storage, redemption2.clone()).unwrap();
    add_to_pending(&mut storage, redemption3.clone()).unwrap();

    // Test get_total_size_of_queue
    let queue_size = get_total_size_of_queue(&storage, "user1".to_string());
    assert_eq!(queue_size, 3);

    // Test get_all_user_redemptions
    let all_redemptions = get_all_user_redemptions(&storage, "user1".to_string()).unwrap();
    assert_eq!(all_redemptions.len(), 3);
    assert!(all_redemptions.contains(&redemption1));
    assert!(all_redemptions.contains(&redemption2));
    assert!(all_redemptions.contains(&redemption3));
}

#[test]
fn test_multiple_users() {
    let mut storage = MockStorage::new();

    // Add redemptions for different users
    let redemption1 = create_test_redemption("user1", 1000);
    let redemption2 = create_test_redemption("user2", 1000);

    add_to_pending(&mut storage, redemption1.clone()).unwrap();
    add_to_pending(&mut storage, redemption2.clone()).unwrap();

    // Test get_all_user_redemptions for each user
    let user1_redemptions = get_all_user_redemptions(&storage, "user1".to_string()).unwrap();
    assert_eq!(user1_redemptions.len(), 1);
    assert_eq!(user1_redemptions[0], redemption1);

    let user2_redemptions = get_all_user_redemptions(&storage, "user2".to_string()).unwrap();
    assert_eq!(user2_redemptions.len(), 1);
    assert_eq!(user2_redemptions[0], redemption2);
}

#[test]
fn test_remove_nonexistent_redemption() {
    let mut storage = MockStorage::new();

    // Try to remove a redemption that doesn't exist
    let result = remove_from_pending(&mut storage, "user1".to_string(), 1000);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        StdError::not_found("Redemption not found").to_string()
    );
}

#[test]
fn test_empty_redemptions_list() {
    let storage = MockStorage::new();

    // Test get_all_user_redemptions for non-existent user
    let redemptions = get_all_user_redemptions(&storage, "user1".to_string()).unwrap();
    assert!(redemptions.is_empty());

    // Test get_total_size_of_queue for non-existent user
    let queue_size = get_total_size_of_queue(&storage, "user1".to_string());
    assert_eq!(queue_size, 0);
}

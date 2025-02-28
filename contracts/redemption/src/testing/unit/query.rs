use crate::{
    query::{query_all_redemptions, query_redemptions, DEFAULT_LIMIT, MAX_LIMIT},
    storage::redemptions::add_to_pending,
};
use cosmwasm_std::coins;
use cosmwasm_std::testing::mock_dependencies;
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
fn test_query_all_redemptions() {
    let mut deps = mock_dependencies();

    // Add test redemptions
    let redemption1 = create_test_redemption("user1", 1000);
    let redemption2 = create_test_redemption("user2", 2000);
    let redemption3 = create_test_redemption("user3", 3000);

    add_to_pending(deps.as_mut().storage, redemption1.clone()).unwrap();
    add_to_pending(deps.as_mut().storage, redemption2.clone()).unwrap();
    add_to_pending(deps.as_mut().storage, redemption3.clone()).unwrap();

    // Test without start_after
    let redemptions = query_all_redemptions(deps.as_ref(), None, Some(2)).unwrap();
    assert_eq!(redemptions.len(), 2);
    assert_eq!(redemptions[0], redemption1);
    assert_eq!(redemptions[1], redemption2);

    // Test with start_after
    let redemptions = query_all_redemptions(
        deps.as_ref(),
        Some(("user1".to_string(), redemption1.timestamp)),
        Some(2),
    )
    .unwrap();

    assert_eq!(redemptions.len(), 2);
    assert_eq!(redemptions[0], redemption2);
    assert_eq!(redemptions[1], redemption3);
}

#[test]
fn test_query_user_redemptions() {
    let mut deps = mock_dependencies();

    // Add multiple redemptions for the same user
    let redemption1 = create_test_redemption("user1", 1000);
    let redemption2 = create_test_redemption("user1", 2000);
    let redemption3 = create_test_redemption("user2", 3000);

    add_to_pending(deps.as_mut().storage, redemption1.clone()).unwrap();
    add_to_pending(deps.as_mut().storage, redemption2.clone()).unwrap();
    add_to_pending(deps.as_mut().storage, redemption3.clone()).unwrap();

    // Test user1's redemptions
    let redemptions = query_redemptions(deps.as_ref(), "user1".to_string(), Some(10)).unwrap();
    assert_eq!(redemptions.len(), 2);
    assert_eq!(redemptions[0], redemption1);
    assert_eq!(redemptions[1], redemption2);

    // Test user2's redemptions
    let redemptions = query_redemptions(deps.as_ref(), "user2".to_string(), Some(10)).unwrap();
    assert_eq!(redemptions.len(), 1);
    assert_eq!(redemptions[0], redemption3);

    // Test non-existent user
    let redemptions = query_redemptions(deps.as_ref(), "user3".to_string(), Some(10)).unwrap();
    assert!(redemptions.is_empty());
}

#[test]
fn test_pagination_limits() {
    let mut deps = mock_dependencies();

    // Add many redemptions
    for i in 0..35 {
        let redemption = create_test_redemption(&format!("user{}", i), i as u64);
        add_to_pending(deps.as_mut().storage, redemption).unwrap();
    }

    // Test default limit
    let redemptions = query_all_redemptions(deps.as_ref(), None, None).unwrap();
    assert_eq!(redemptions.len(), DEFAULT_LIMIT as usize);

    // Test max limit
    let redemptions = query_all_redemptions(deps.as_ref(), None, Some(100)).unwrap();
    assert_eq!(redemptions.len(), MAX_LIMIT as usize);
}

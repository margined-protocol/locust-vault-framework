use crate::storage::state::Config;

use cosmwasm_std::testing::mock_dependencies;
use interface::redemption::FundInfo;
fn create_test_fund(address: &str, metadata: &str) -> FundInfo {
    FundInfo {
        address: address.to_string(),
        metadata: metadata.to_string(),
    }
}

#[test]
fn test_invalid_admin_address() {
    let deps = mock_dependencies();
    let config = Config {
        admin: "invalid/address".to_string(),
        whitelisted_funds: vec![],
    };

    assert!(config.validate(&deps.as_ref()).is_err());
}

#[test]
fn test_invalid_fund_address() {
    let deps = mock_dependencies();
    let config = Config {
        admin: "admin0001".to_string(),
        whitelisted_funds: vec![create_test_fund("invalid/address", "metadata")],
    };

    assert!(config.validate(&deps.as_ref()).is_err());
}

#[test]
fn test_duplicate_funds() {
    let deps = mock_dependencies();
    let config = Config {
        admin: "admin0001".to_string(),
        whitelisted_funds: vec![
            create_test_fund("fund0001", "metadata1"),
            create_test_fund("fund0001", "metadata2"),
        ],
    };

    assert!(config.validate(&deps.as_ref()).is_err());
}

# Redemption Manager Contract

The redemption manager contract enables funds from processed redemptions to be held on behalf of a user until they are claimed.

## Architecture

```mermaid
classDiagram
    class RedemptionManager {
        +Config config
        +sendRedemption()
        +claimRedemption()
        +updateConfig()
        +queryRedemptions()
    }

    class Config {
        +String admin
        +Vec~FundInfo~ whitelisted_funds
    }

    class PendingRedemption {
        +String user
        +Vec~Coin~ funds
        +u64 timestamp
        +String source
    }

    class Storage {
        +Map~(String,u64),PendingRedemption~ redemptions
        +Index~user~ redemptions_by_user
    }

    RedemptionManager --> Config
    RedemptionManager --> Storage
    Storage --> PendingRedemption
```

## Flow Diagrams

### Redemption Process

```mermaid
sequenceDiagram
    participant Fund
    participant RedemptionManager
    participant User

    Fund->>RedemptionManager: Send Redemption
    Note over RedemptionManager: Validate Fund
    Note over RedemptionManager: Store Redemption

    User->>RedemptionManager: Claim Redemption
    Note over RedemptionManager: Verify Ownership
    RedemptionManager->>User: Transfer Funds
```

### Fund Management

```mermaid
sequenceDiagram
    participant Admin
    participant RedemptionManager
    participant Fund

    Admin->>RedemptionManager: Add Fund to Whitelist
    Note over RedemptionManager: Validate Fund Info
    Note over RedemptionManager: Update Config

    Fund->>RedemptionManager: Attempt Redemption
    Note over RedemptionManager: Check Whitelist
    alt Fund Whitelisted
        RedemptionManager->>RedemptionManager: Process Redemption
    else Fund Not Whitelisted
        RedemptionManager->>Fund: Reject Transaction
    end
```

## Key Components

1. **Whitelisted Funds**

   - Maximum 100 funds
   - Admin controlled
   - Validated addresses

2. **Redemption Queue**

   - FIFO processing
   - User-specific tracking
   - Timestamp ordering

3. **Security Features**

   - Fund validation
   - User verification
   - Amount verification

4. **Admin Controls**
   - Fund whitelist management
   - Configuration updates
   - Emergency controls

**NOTE:** All admin functionality for production contracts should be managed by a multisig wallet.

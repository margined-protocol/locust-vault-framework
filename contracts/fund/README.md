# Fund Vault

The fund vault manages deposits and redemptions into the vault. It supports deposit of single and multiple assets. It is managed by two accounts, the `admin` and `controller`.

## Architecture

```mermaid
classDiagram
    class FundVault {
        +Config config
        +State state
        +deposit(tokens)
        +withdraw(amount)
        +repay(tokens, profit)
        +updateConfig(config)
        +createRedemption(amount)
        +cancelRedemption()
    }

    class Config {
        +String admin
        +String controller
        +String treasury
        +String redemption_contract
        +Uint128 strategy_cap
        +Option~Decimal~ float
        +String strategy_denom
        +String token0
        +Option~String~ token1
        +Decimal management_fee_rate
        +Decimal performance_fee_rate
    }

    class State {
        +bool is_open
        +bool is_paused
        +Timestamp last_pause
        +Timestamp last_claim
        +Uint128 total_staked_tokens
        +Vec~Coin~ total_withdrawn_tokens
        +Vec~Coin~ pending_management_fees
    }

    FundVault --> Config
    FundVault --> State
```

## Flow Diagrams

### Deposit Flow

```mermaid
sequenceDiagram
    participant User
    participant FundVault
    participant Strategy

    User->>FundVault: Deposit Tokens
    FundVault->>FundVault: Calculate Shares
    FundVault->>User: Mint Share Tokens
    Note over FundVault: Tokens available for strategy
```

### Withdrawal Flow

```mermaid
sequenceDiagram
    participant User
    participant FundVault
    participant RedemptionManager
    participant Strategy

    User->>FundVault: Request Withdrawal
    alt Has Float Available
        FundVault->>User: Instant Withdrawal
        FundVault->>FundVault: Burn Share Tokens
    else No Float
        FundVault->>RedemptionManager: Create Redemption
        FundVault->>FundVault: Burn Share Tokens
        Note over RedemptionManager: User can claim later
    end
```

## Key Components

The vault has several key components:

1. **Admin Role**

   - Manages vault configuration
   - Sets strategy cap
   - Controls float settings

2. **Controller Role**

   - Manages balance sheet
   - Withdraws and repays funds
   - Typically the strategy contract

3. **Float Mechanism**

   - Enables instant withdrawals
   - Limited by percentage of assets
   - Configurable by admin

4. **Fee Structure**
   - Management fees
   - Performance fees
   - Treasury collection

**NOTE:** All admin functionality for production vaults should be managed by a multisig wallet.

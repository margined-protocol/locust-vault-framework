# Strategy Contract

The strategy contract manages a balance sheet of inventory and executes trading strategies through authorized actions via `authz`. It coordinates with the fund vault for withdrawals and repayments.

## Architecture

```mermaid
classDiagram
    class Strategy {
        +Config config
        +executeStrategy()
        +withdrawFunds()
        +repayFunds()
        +queryPrice()
        +updateConfig()
    }

    class Config {
        +String admin
        +String controller
        +Option~String~ vault
        +String token0
        +Option~String~ token1
        +PoolInfo pool_info
        +Vec~String~ grants
    }

    class PoolInfo {
        +Osmosis osmosis
        +Astroport astroport
        +Slinky slinky
        +Drop drop
    }

    Strategy --> Config
    Config --> PoolInfo
```

## Flow Diagrams

### Strategy Execution Flow

```mermaid
sequenceDiagram
    participant Controller
    participant Strategy
    participant PriceSource
    participant Vault

    Controller->>Strategy: Execute Strategy
    Strategy->>PriceSource: Query Price
    PriceSource->>Strategy: Return Price
    Strategy->>Vault: Withdraw Funds
    Note over Strategy: Execute Trading Logic
    Strategy->>Vault: Repay with Profits
```

### Price Integration Flow

```mermaid
sequenceDiagram
    participant Strategy
    participant PriceSource
    participant Vault

    Note over PriceSource: Osmosis/Astroport/Slinky/Drop
    Strategy->>PriceSource: Query Price
    PriceSource->>Strategy: Return Price
    Strategy->>Strategy: Validate Price
    Strategy->>Vault: Use Price for Calculations
```

## Key Components

1. **Admin Management**

   - Configures strategy parameters
   - Manages grants
   - Controls price source settings

2. **Controller Functions**

   - Executes strategy operations
   - Manages fund withdrawals/repayments
   - Interacts with price sources

3. **Price Sources**

   - Osmosis TWAPs
   - Astroport Observations
   - Slinky Integration
   - Drop Protocol

4. **Authorization**
   - Uses `authz` for permissions
   - Limited to granted operations
   - Secure execution model

**NOTE:** All admin functionality for production contracts should be managed by a multisig wallet.

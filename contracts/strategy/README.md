# Strategy Contract

The strategy contract is able to manage a balance sheet of inventory but is limited to performing actions for which is has permission to perform via `authz`. It withdraws funds from the fund vault and repays them when profit is realised.

It is managed by an `admin` and a `controller`. The `admin` is responsible for managing the configuration of the strategy, e.g. `grants`. The `controller` able to execute any `grants` and perform withdrawal and repayment of funds to the defined vault.

Further the `strategy contract` is a proxy for a variety of price sources. Currently it supports only `Osmosis TWAPs` and `Astroport Observations` but is being extended to support other price sources such as `Slinky` and `Pyth`. being able to support different price sources allows the `strategy contract` to be more flexible and able to support a variety of strategies and removes the need for the `fund vault` to care about where it is deployed.

**NOTE:** All admin functionality for production contracts should be managed by a multisig wallet.

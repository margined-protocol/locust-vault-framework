# Fund Vault

The fund vault simply manages deposit and redemptions into the vault. It supports deposit of single and multiple assets. It is managed by two accounts, the `admin` and `controller`.

The `admin` is responsible for managing the configuration of the vault, e.g. `strategy_cap`. The `controller` is responsible for managing balance sheet of the vault, i.e. withdrawing and repaying funds, and is designed to be the `strategy contract`.

The `admin` is able to update the `strategy_cap` which limits the amount of assets that can be deposited in the `strategy contract`. Additionally the `float` can be set, which limits the percentage of assets that can be withdrawn from the vault. The `float` is used to enable redemptions to be processed without the need to repay the `strategy contract`, though it is obviously limited to available assets.

**NOTE:** All admin functionality for production vaults should be managed by a multisig wallet.

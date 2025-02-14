# Redemption Manager Contract

The redemption manager contract enables funds from processed redemptions to be held on behalf of a user until they are claimed.

The contract can receive funds only from a whitelisted `Fund` contract and funds can subsequently only be claimed by the assigned user.

**NOTE:** All admin functionality for production contracts should be managed by a multisig wallet.

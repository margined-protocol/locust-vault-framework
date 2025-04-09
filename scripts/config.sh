#!/bin/sh

# ------------------------
# Chain variables
# ------------------------
# The executable to use
BINARY="osmosisd"

# The RPC Server to use
NODE="https://osmosis-testnet-rpc.polkachu.com:443"

# The chain id to use - see https://github.com/cosmos/chain-registry/
CHAIN_ID="osmo-test-5"

# Gas settings
GAS="auto"
GAS_PRICES="0.0025uosmo"
GAS_ADJUSTMENT="1.3"

# Output format
OUTPUT_FORMAT="json"

# Admin Account
ADMIN="osmo1pwaceq0ysz3wgrzfxdd45gzpe5ne8j46eevf87"

# Default Sender Account
FROM="osmo1pwaceq0ysz3wgrzfxdd45gzpe5ne8j46eevf87"


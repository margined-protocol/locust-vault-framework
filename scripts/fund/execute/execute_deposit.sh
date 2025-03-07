#!/bin/sh

# Load shared config from the parent directory
. "$(dirname "$0")/../../config.sh"

# ------------------------
# Contract Execution Variables
# ------------------------
# Allow contract address to be set by first argument, otherwise use default
CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"  # Replace with your default contract address

# Allow deposit amount to be set by second argument, otherwise use default (1 OSMO)
DEPOSIT_AMOUNT="${2:-10000000uosmo}"  # Default to 10,000,000 uosmo (10 OSMO)

# Construct the JSON payload
PAYLOAD="{
  \"deposit\": {
    \"amount\": \"1\"
  }
}"

# Execute the transaction
"$BINARY" tx wasm execute "$CONTRACT_ADDRESS" \
  "$PAYLOAD" \
  --from="$FROM" \
  --amount "${DEPOSIT_AMOUNT}" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"


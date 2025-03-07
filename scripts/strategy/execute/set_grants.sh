#!/bin/sh
# Load shared variables
. "$(dirname "$0")/../../config.sh"

CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"

# Define the grants as a properly formatted JSON array
GRANTS="
  \"/cosmwasm.wasm.v1.MsgExecuteContract\""

# Construct the JSON payload properly
PAYLOAD="{
  \"set_grants\": {
    \"grants\": [${GRANTS}]
  }
}"

# Execute the transaction
"$BINARY" tx wasm execute "$CONTRACT_ADDRESS" "$PAYLOAD" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

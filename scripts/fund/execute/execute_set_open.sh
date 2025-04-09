#!/bin/sh
# Load shared variables
. "$(dirname "$0")/../../config.sh"

CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"

PAYLOAD="{
  \"vault_extension\": {
    \"vaultenator\": {
      \"set_open\": {}
    }
  }
}"

# Execute the transaction
"$BINARY" tx wasm execute "$CONTRACT_ADDRESS" \
  "$PAYLOAD" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

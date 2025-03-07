#!/bin/sh

# Load shared variables
. "$(dirname "$0")/../../config.sh"

CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"

# Construct payload
PAYLOAD='{"vault_extension": {"vaultenator": {"version": {}}}}'

# Execute the query transaction
"$BINARY" query wasm contract-state smart "$CONTRACT_ADDRESS" "$PAYLOAD" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID" | jq .

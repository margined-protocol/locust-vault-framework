#!/bin/sh
# Load shared variables
. "$(dirname "$0")/../../config.sh"

# Ensure contract address and grantee are set
CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"  # Replace with the default contract
CONTROLLER="${2:-$FROM}"

# Construct the JSON payload properly (Vec<Coin> format)
PAYLOAD="{
  \"withdraw\": {
    \"tokens_to_withdraw\": [
      {
        \"amount\": \"1000000\",
        \"denom\": \"uosmo\"
      }
    ]
  }
}"

# Generate the transaction JSON file
"$BINARY" tx wasm execute "$CONTRACT_ADDRESS" "$PAYLOAD" \
  --from="$CONTROLLER" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output=json \
  --node="$NODE" \
  --chain-id="$CHAIN_ID" \

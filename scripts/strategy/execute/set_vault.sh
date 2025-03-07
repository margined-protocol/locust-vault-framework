#!/bin/sh
# 
# Load shared variables
. "$(dirname "$0")/../../config.sh"

# Ensure contract address is set via argument
CONTRACT_ADDRESS="${1:-YOUR_CONTRACT_ADDRESS}"

# Allow VAULT_ADDRESS to be passed as an argument (or set default)
VAULT_ADDRESS="${2:-YOUR_VAULT_ADDRESS}"

# Ensure VAULT_ADDRESS is provided
if [ -z "$VAULT_ADDRESS" ]; then
  echo "Error: VAULT_ADDRESS is required" >&2
  exit 1
fi

# Construct the JSON payload properly
PAYLOAD="{
  \"set_vault\": {
    \"vault\": \"$VAULT_ADDRESS\"
  }
}"

# Print payload for debugging (optional)
echo "Executing set_vault with payload: $PAYLOAD"

# Execute the transaction
"$BINARY" tx wasm execute "$CONTRACT_ADDRESS" "$PAYLOAD" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"


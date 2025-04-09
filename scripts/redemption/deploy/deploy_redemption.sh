#!/bin/sh

# Load shared variables
. "$(dirname "$0")/../../config.sh"


# ------------------------
# Contract Variables
# ------------------------
CODE_ID="12227" 
LABEL="margined-locust-redemption-queue"

# Construct the JSON payload
PAYLOAD="{
  \"admin\": \"$ADMIN\",
  \"whitelisted_funds\": []
}"

# Execute the transaction
"$BINARY" tx wasm instantiate "$CODE_ID" \
  "$PAYLOAD" \
  --label="$LABEL" \
  --admin="$ADMIN" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

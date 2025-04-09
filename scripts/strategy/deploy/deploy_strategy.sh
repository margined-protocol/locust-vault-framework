#!/bin/sh

# Load shared variables
. "$(dirname "$0")/../../config.sh"

# ------------------------
# Contract Variables
# ------------------------
# Code ID is the contract code to instantiate
# See https://docs.margined.io/resources/contracts
CODE_ID="1453"

# Controller is the off-chain executor for this strategy
CONTROLLER="osmo17fqss433rmea7xkcml2sysmvlqrqnvujk3v755"

# Token0 is the denom used in the strategy
TOKEN0="uosmo"

# Token1 is the second denom used in the strategy
# For single denom strategies leave this string blank
TOKEN1="" 

# Label is the label given to the contract
LABEL="locust-strategy-levana-uosmo"

# Grants are the Authz grants issued to the controller account
GRANTS="
  \"/cosmwasm.wasm.v1.MsgExecuteContract\""

# Pool info is the configuration for an on-chain oracle price
POOL_INFO="
  \"slinky\": {
    \"base\": \"OSMO\",
    \"quote\": \"USD\",
    \"timeout\": 900
  }"

# ------------------------
# DO NOT EDIT BELOW HERE
# ------------------------
# Start constructing JSON payload
PAYLOAD="{
  \"admin\": \"$ADMIN\",
  \"controller\": \"$CONTROLLER\",
  \"token0\": \"$TOKEN0\""

# Append token1 only if it's set
if [ -n "$TOKEN1" ]; then
  PAYLOAD="$PAYLOAD,
  \"token1\": \"$TOKEN1\""
fi

# Append remaining JSON structure
PAYLOAD="$PAYLOAD,
  \"grants\": [$GRANTS],
  \"pool_info\": { $POOL_INFO }
}"

# Execute the transaction
"$BINARY" tx wasm instantiate "$CODE_ID" \
  "$PAYLOAD" \
  --label="$LABEL" \
  --admin "$ADMIN" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

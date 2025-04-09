#!/bin/sh
# ------------------------
# Script variables
# ------------------------
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Load shared variables
. "$(dirname "$0")/../../config.sh"

# ------------------------
# Contract Variables
# ------------------------
# Contract Path is the where the compiled contract is
CONTRACT_PATH="$ROOT_DIR/artifacts/fund.wasm"

# ------------------------
# DO NOT EDIT BELOW HERE
# ------------------------
# Execute the transaction
"$BINARY" tx wasm store "$CONTRACT_PATH" \
  --from="$FROM" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

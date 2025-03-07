#!/bin/sh

# Load shared variables
. "$(dirname "$0")/../../config.sh"

# ------------------------
# Contract Variables
# ------------------------

# Code ID is the contract code to instantiate
# See https://docs.margined.io/resources/contracts
CODE_ID="12226"

# Controller is an instance of a strategy deployment that manages funds in this strategy
CONTROLLER="osmo1sup8eazmeth3gcs6ydzlatnxlw9uvpamrt9d0nyud0scrv2n8a7sju7haf"

# Redemption contract is the contract used to manage redemptions
REDEMPTION_CONTRACT="osmo1l0x5k0r6rmc75dk22mdljnjft4y99qfagfaj2l5a3lnfg53f5pvqn99kf5"

# Treasury account, where fees should be sent
TREASURY="osmo12qk679ejsyev85d9859w7g0gk6snv65m3vqrm9"

# The strategy deposit cap, calculated by valuing deposits against a TWAP
STRATEGY_CAP="500000000"

# Float is the percentage that will remain on the fund contract for withdrawals
FLOAT="0.02"

# Token0 is the token accepted for deposits
TOKEN0="uosmo"

# Token1 is the token accepted in addition to Token0 for deposits
# For single sided strategies leave this string blank
TOKEN1=""

# Performance fee
PERFORMANCE_FEE_RATE="0.15"

# Management Fee
MANAGEMENT_FEE_RATE="0.02"

# Vault Type is simply a string
VAULT_TYPE="fund"

# Label is the label for the vault
LABEL="fund-vault-uosmo"

# Amount is the funds sent with the transaction
AMOUNT="1000${TOKEN0}"

# ------------------------
# DO NOT EDIT BELOW HERE
# ------------------------
# Construct JSON payload dynamically
PAYLOAD="{
  \"admin\": \"$ADMIN\",
  \"controller\": \"$CONTROLLER\",
  \"treasury\": \"$TREASURY\",
  \"strategy_cap\": \"$STRATEGY_CAP\",
  \"float\": \"$FLOAT\",
  \"token0\": \"$TOKEN0\",
  \"performance_fee_rate\": \"$PERFORMANCE_FEE_RATE\",
  \"management_fee_rate\": \"$MANAGEMENT_FEE_RATE\",
  \"redemption_contract\": \"$REDEMPTION_CONTRACT\",
  \"vault_type\": \"$VAULT_TYPE\""

# Append token1 only if it's set
if [ -n "$TOKEN1" ]; then
  PAYLOAD="$PAYLOAD,
  \"token1\": \"$TOKEN1\""
fi

# Close JSON payload
PAYLOAD="$PAYLOAD
}"

# Execute the transaction
"$BINARY" tx wasm instantiate "$CODE_ID" \
  "$PAYLOAD" \
  --label="$LABEL" \
  --admin "$ADMIN" \
  --from="$FROM" \
  --amount "$AMOUNT" \
  --gas="$GAS" \
  --gas-prices="$GAS_PRICES" \
  --gas-adjustment="$GAS_ADJUSTMENT" \
  --output="$OUTPUT_FORMAT" \
  --node="$NODE" \
  --chain-id="$CHAIN_ID"

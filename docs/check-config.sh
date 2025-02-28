#!/bin/bash

NODE=https://neutron-rpc.publicnode.com:443
CHAIN_ID="neutron-1"

# Function to query contract state
query_contract() {
    local contract=$1
    local query=$2
    local label=$3
    
    echo -n "$label: "
    # Extract just the version for version queries
    neutrond query wasm contract-state smart $contract "$query" --node=$NODE --output=json | jq -r '.data.version // .data.vault_extension.vaultenator.version // "N/A"'
}

# Store contract pairs in an array
# Format: "PAIR_NAME|CONTRACT_1|CONTRACT_2"
declare -a CONTRACT_PAIRS=(
    "ATOM (Neutron)|neutron1puedrclm6rn33x3zv66xg6m23qcdagayqua6jj2wqzvfznlqef8qe53wr2|neutron1ajk4hcvtf48qwt773v8cpwraq2qtj9kum6x6twua8jzv7863y8csqs64ca"
    "TIA (Neutron)|neutron14q3umuuvyv6mndd5acuc3n8u5mlvrrq3kkzrputu3rkhz8nd2uzqmfl4v6|neutron1kz5qqv5jmz7q6y96ftq0ew9023m3tmhpex788z4slhm0xuax7vpqz72jev"
    "ATOM<>dATOM|neutron1f99ujxefjr4jqmskc7hvg09am6pdq2j2c5049xwl0de4cavc4rfsl866y0|neutron1cgd08p87rl70psgqneua6nv4s3hzhvtgeykg7hfzqk0x48u589wsa9ss2l"
    "wBTC.axl<>USDC|neutron1egc0ujxyqh8p35nxrvxd04uq0z9536k6fvwzwecfgjj9yg7wkdgq2jzj38|neutron1tvg8hupp64s7aycmhdw638w9g5zkuhc643ywcg9upa0md0kcm4us9eh0kc"
    "NTRN<>USDC|neutron1t0fl9k43g86sv60ghx9vtwed9rpgtf49rxzm05ff477j23h52c6s0urdc7|neutron13tyej6xvgj4uc4c2xxgktjkgllzgngaksqkvyv42lx6y82teum6q3307xw"
    "TIA<>USDC|neutron1wv8pl7tsatzx6n9yaqfksvu5y0x7j50g6mhy636udwfn3vyqp0hsu7g8yk|neutron1sj9ax77exvyc86dv30399dw54yehnfajaltj8ugf70vswus9xuks4ldwtu"
    "ATOM<>USDC|neutron1krqwpk0kmphl93kykavp2fnr88g5rnrpk40c34a55yrl00tmfz0s99ewc6|neutron1rtvdz9u2zdwtadc8zglawau3f6jrh8jefxjwayl9jc42e24t5rlsflpchs"
    "Deprecated wBTC<>USDC|neutron17fyzkafg4scrd6xu0sp9llrl6hazegza7yer4erlea0kvk30yxsqk2xqfd|neutron1me4fuchq3pgle46dvdxsgvpz02z605gkr0sgs6uwew25cpgg3ydsfg8zms"
)

# Define queries
CONFIG_QUERY_2='{"config": {}}'
VERSION_QUERY_1='{"vault_extension": {"vaultenator": {"version": {}}}}'

# Loop through each contract pair
for pair in "${CONTRACT_PAIRS[@]}"; do
    # Split the pair string into components
    IFS="|" read -r PAIR_NAME CONTRACT_1 CONTRACT_2 <<< "$pair"
    
    echo ""
    echo "========== $PAIR_NAME =========="
    
    # Query first contract
    query_contract "$CONTRACT_1" "$VERSION_QUERY_1" "Version (Vault)"
    
    # Query second contract
    query_contract "$CONTRACT_2" "$CONFIG_QUERY_2" "Config (Strategy)"
    
    echo "================================="
done

# Instructions for adding new contract pairs:
# To add a new contract pair, add a new line to the CONTRACT_PAIRS array above
# in the format: "PAIR_NAME|VAULT_CONTRACT_ADDRESS|STRATEGY_CONTRACT_ADDRESS"
```bash
NODE=https://neutron-rpc.publicnode.com:443
CHAIN_ID="neutron-1"
CLIENT=neutrond
NODE=https://rpc.osmosis.zone:443
CHAIN_ID="osmosis-1"
CLIENT=osmosisd

# Redemption Queue Osmosis
CONTRACT_ADDRESS=osmo1g63cnjmq2spfagpp03et04xpgklnfllzzc23xqm953huqhrgr7dsaytq8k
```

## Executes

```bash
$CLIENT tx wasm execute $CONTRACT_ADDRESS "{\"update_config\": {\"add_fund\": {\"address\":\"neutron1hd4le7ndpxfjzw9vny903pdm5pjkqe7rr9utn9ddxsdak5fdrf9smph7d0\",\"metadata\":\"Hydro Hedging Strategy\"}}}" --from=deployer --gas=auto --gas-prices 0.007untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

## Queries

```bash
$CLIENT q wasm contract-state smart $CONTRACT_ADDRESS "{\"all_redemptions\": {}}" --node=$NODE --output=json | jq .
$CLIENT q wasm contract-state smart $CONTRACT_ADDRESS "{\"config\": {}}" --node=$NODE --output=json | jq .
```

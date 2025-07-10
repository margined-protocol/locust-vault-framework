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
$CLIENT tx wasm execute $CONTRACT_ADDRESS "{\"update_config\": {\"add_fund\": {\"address\":\"neutron1f99ujxefjr4jqmskc7hvg09am6pdq2j2c5049xwl0de4cavc4rfsl866y0\",\"metadata\":\"ATOM<>dATOM MM\"}}}" --from=deployer --gas=auto --gas-prices 0.007untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

## Queries

```bash
$CLIENT q wasm contract-state smart $CONTRACT_ADDRESS "{\"all_redemptions\": {}}" --node=$NODE --output=json | jq .
$CLIENT q wasm contract-state smart $CONTRACT_ADDRESS "{\"config\": {}}" --node=$NODE --output=json | jq .
```

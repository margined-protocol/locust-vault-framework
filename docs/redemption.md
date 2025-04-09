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

## Queries

```bash
$CLIENT q wasm contract-state smart $CONTRACT_ADDRESS "{\"all_redemptions\": {}}" --node=$NODE --output=json | jq .
```

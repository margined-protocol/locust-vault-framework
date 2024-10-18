# Helper Functions

**NOTE:** Find the relevant docs for contract locations [here](https://docs.margined.io/resources/contracts).

## Setup

### Mainnet - Osmosis

```
NODE=https://rpc.osmosis.zone:443
CHAIN_ID="osmosis-1"
CONTRACT_NAME=locust_vault.wasm
CONTRACT_ADDRESS=osmo1q3w9kedgtc8sdh7xlcr77ydv3qu7fs2e3q6xznysp2lrdfyz9xyqv26yqv
USER_ADDRESS=osmo1q3w9kedgtc8sdh7xlcr77ydv3qu7fs2e3q6xznysp2lrdfyz9xyqv26yqv
CONTRACT_ADDRESS=osmo16s3sxs5886p42kteunp6370pken2n5ukzszz0trkr39epqtawn2qk4r9l5
USER_ADDRESS=osmo16s3sxs5886p42kteunp6370pken2n5ukzszz0trkr39epqtawn2qk4r9l5
POOL_ID=1922
CODE_ID=922
```

### Mainnet - Neutron

```
NODE=https://neutron-rpc.polkachu.com:443
CHAIN_ID="neutron-1"
CONTRACT_NAME=strategy-astroport.wasm
CONTRACT_ADDRESS=neutron1ajk4hcvtf48qwt773v8cpwraq2qtj9kum6x6twua8jzv7863y8csqs64ca
CODE_ID=6947
```

### Testnet - Neutron

```
NODE=https://neutron-testnet-rpc.polkachu.com:443
CHAIN_ID="pion-1"
CONTRACT_NAME=strategy-astroport.wasm
CONTRACT_NAME=fund.wasm
CONTRACT_ADDRESS=neutron1yyyzx2f0p9t0huf5fy0aqlg8tq34vdv3sxehzzkvq4vw04qqsm0qslfnrx
CONTRACT_ADDRESS=neutron1pausm8t24783f764gku6cmth22z478t608w9t9c2heem74vcxxnqvq7t7f
CODE_ID=6947
```

## Wasm

### Store

```bash
osmosisd tx wasm store ./artifacts/$CONTRACT_NAME --from=deployer --gas=auto --gas-prices 0.003uosmo --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
neutrond tx wasm store ./artifacts/$CONTRACT_NAME --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Instantiate

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1y3fzmdmlqrhxfjh570cdh74nve5e33apwl2j0t\", \"token0\": \"factory/neutron1nm80734yaw223ewvn30s32n2nfq6tdd0vzzdnk/ibc/usdc\", \"token1\": \"factory/neutron1nm80734yaw223ewvn30s32n2nfq6tdd0vzzdnk/umuntrn\",\"grants\": [\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\"], \"pool_info\": {\"astroport\": {\"pool_address\": \"neutron13zhre5d8fnply63j58rvhhd27cuzrhq7kpmrvn9cg56xt27fs7jqvlgtg0\", \"token0\": \"factory/neutron18mcny8tjmy7rw4x3j9ph86gk0hk47dr5crnfc4njqaxc6xvv9a3sumj0cy/astro\", \"token1\": \"untrn\"}}}" --label="margined-strategy-contract-astroport" --admin deployer --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Migrate

```bash
osmosisd tx wasm migrate $CONTRACT_ADDRESS $CODE_ID '{}'  --from=deployer --gas=auto --gas-prices 0.0025uosmo --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
neutrond tx wasm migrate $CONTRACT_ADDRESS $CODE_ID '{}'  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Execute

#### Set Vault

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"set_vault\": {\"vault\": \"neutron148hshtgsu503zgnegc2zh2x5f8cmcax59fcj3fe2wu7yrlh6yx4scck99m\"}}"  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Set Vault

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"set_grants\": {\"grants\": [\"/cosmwasm.wasm.v1.MsgExecuteContract\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\"]}}"  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Query

#### Config

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"config\": {}}" --node=$NODE --output=json | jq .
```

#### Grants

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"grants\": {}}" --node=$NODE --output=json | jq .
```

#### Spot Price

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"spot_price\": {}}" --node=$NODE --output=json | jq .
```

#### Twap Price

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"twap_price\": {\"duration\": 2}}" --node=$NODE --output=json | jq .
```

#### Astroport

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"observe\": {\"seconds_ago\": 60}}" --node=$NODE --output=json | jq .
```

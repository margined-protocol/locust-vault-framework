# Helper Functions

**NOTE:** Find the relevant docs for contract locations [here](https://docs.margined.io/resources/contracts).

## Setup

```
NODE=https://neutron-rpc.publicnode.com:443
CHAIN_ID="neutron-1"
CONTRACT_NAME=fund.wasm
CONTRACT_NAME=strategy-slinky.wasm
CONTRACT_NAME=strategy-astroport.wasm
CONTRACT_ADDRESS=neutron1puedrclm6rn33x3zv66xg6m23qcdagayqua6jj2wqzvfznlqef8qe53wr2
```

### Testnet

```
NODE=https://neutron-testnet-rpc.polkachu.com:443
CHAIN_ID="pion-1"
CONTRACT_NAME=fund-aarch64.wasm
CONTRACT_ADDRESS=neutron148hshtgsu503zgnegc2zh2x5f8cmcax59fcj3fe2wu7yrlh6yx4scck99m
```

## Wasm

### Store

```bash
neutrond tx wasm store ./artifacts/$CONTRACT_NAME --from=deployer-neutron --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Instantiate - Fund

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1rtvdz9u2zdwtadc8zglawau3f6jrh8jefxjwayl9jc42e24t5rlsflpchs\",  \"treasury\": \"neutron1d9uwvv4w4n8yfnng8enl643vml4xwke63m63gl\", \"strategy_cap\": \"1000000000\", \"float\": \"0.02\", \"token0\": \"ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9\", \"token1\": \"ibc/B559A80D62249C8AA07A380E2A2BEA6E5CA9A6F079C912C3A9E9B494105E4F81\", \"performance_fee_rate\": \"0.15\", \"management_fee_rate\": \"0.02\", \"vault_type\": \"fund\"}" --label="margined-locust-fund-vault-{$DENOMS}" --admin deployer-neutron --from=deployer-neutron --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID  --amount 100000ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9
```

### Instantiate - Strategy

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1nz852flh6np9xlg9ju3ka6w5txezsxt0j4lypn\", \"token0\": \"ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9\", \"token1\": \"ibc/B559A80D62249C8AA07A380E2A2BEA6E5CA9A6F079C912C3A9E9B494105E4F81\", \"grants\": [\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgWithdrawFilledLimitOrder\",\"/neutron.dex.MsgCancelLimitOrder\"], \"pool_info\": {\"slinky\": {\"timeout\": 900, \"base\": \"ATOM\",\"quote\": \"USD\"}}}" --label="margined-locust-strategy-{$DENOMS}" --admin deployer-neutron --from=deployer-neutron --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Execute

#### Crank

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"crank\": {}}}}"  --from=deployer --gas=auto --gas-prices 0.003untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Deposit

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"deposit\": {\"amount\": \"1\"}}" --from=deployer --gas=auto --gas-prices 0.007untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID --amount 10000000ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9
```

#### Redeem

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"redeem\": {\"amount\": \"0\"}}" --from=liquidity-provider --gas=auto --gas-prices 0.003untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID --amount 15118336factory/osmo16s3sxs5886p42kteunp6370pken2n5ukzszz0trkr39epqtawn2qk4r9l5/lsd-vault-1252
```

#### Set Open

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"set_open\": {}}}}"  --from=deployer --gas=auto --gas-prices 0.003untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Set Vault

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"set_vault\": {\"vault\":\"neutron1krqwpk0kmphl93kykavp2fnr88g5rnrpk40c34a55yrl00tmfz0s99ewc6\"}}"  --from=deployer-neutron --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Update Config

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"update_config\": {\"new_config\": {\"strategy_cap\": \"500000000000\"}}}}}"  --from=deployer --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"update_config\": {\"new_config\": {\"controller\": \"neutron1ajk4hcvtf48qwt773v8cpwraq2qtj9kum6x6twua8jzv7863y8csqs64ca\"}}}}}"  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Migrate

```bash
neutrond tx wasm migrate $CONTRACT_ADDRESS $CODE_ID '{}'  --from=deployer --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Query

#### Config

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"config\": {}}}}" --node=$NODE --output=json | jq .
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"config\": {}}" --node=$NODE --output=json | jq .
```

#### State

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"state\": {}}}}" --node=$NODE --output=json | jq .
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"state\": {}}" --node=$NODE --output=json | jq .
```

#### Estimate Vault Assets

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"estimate_vault_assets\": { \"amount\": \"1000\" }}}}" --node=$NODE --output=json | jq .
```

#### Total Vault Token Supply

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"total_vault_token_supply\": {}}" --node=$NODE --output=json | jq .
```

#### Version

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"version\": {}}}}" --node=$NODE --output=json | jq .
```

#### Withdrawable Amount

```bash
neutrond query wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"withdrawable_amount\": {}}}}" --node=$NODE --output=json | jq .
```

### Other

```bash
neutrond q marketmap market-map --node=$NODE --output json | jq .
neutrond q oracle currency-pairs  --node=$NODE --output json | jq .
```

4.911132252

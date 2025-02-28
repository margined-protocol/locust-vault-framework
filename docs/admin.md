# Helper Functions

**NOTE:** Find the relevant docs for contract locations [here](https://docs.margined.io/resources/contracts).

## Setup

```bash
NODE=https://neutron-rpc.publicnode.com:443
CHAIN_ID="neutron-1"
CONTRACT_NAME=fund.wasm
CONTRACT_NAME=strategy-slinky.wasm
CONTRACT_NAME=strategy-astroport.wasm

# ATOM (Neutron)
CONTRACT_ADDRESS=neutron1puedrclm6rn33x3zv66xg6m23qcdagayqua6jj2wqzvfznlqef8qe53wr2
CONTRACT_ADDRESS=neutron1ajk4hcvtf48qwt773v8cpwraq2qtj9kum6x6twua8jzv7863y8csqs64ca

# TIA (Neutron)
CONTRACT_ADDRESS=neutron14q3umuuvyv6mndd5acuc3n8u5mlvrrq3kkzrputu3rkhz8nd2uzqmfl4v6
CONTRACT_ADDRESS=neutron1kz5qqv5jmz7q6y96ftq0ew9023m3tmhpex788z4slhm0xuax7vpqz72jev

# ATOM<>dATOM
CONTRACT_ADDRESS=neutron1f99ujxefjr4jqmskc7hvg09am6pdq2j2c5049xwl0de4cavc4rfsl866y0
CONTRACT_ADDRESS=neutron1cgd08p87rl70psgqneua6nv4s3hzhvtgeykg7hfzqk0x48u589wsa9ss2l

# wBTC.axl<>USDC
CONTRACT_ADDRESS=neutron1egc0ujxyqh8p35nxrvxd04uq0z9536k6fvwzwecfgjj9yg7wkdgq2jzj38
CONTRACT_ADDRESS=neutron1tvg8hupp64s7aycmhdw638w9g5zkuhc643ywcg9upa0md0kcm4us9eh0kc

#NTRN<>USDC
CONTRACT_ADDRESS=neutron1t0fl9k43g86sv60ghx9vtwed9rpgtf49rxzm05ff477j23h52c6s0urdc7
CONTRACT_ADDRESS=neutron13tyej6xvgj4uc4c2xxgktjkgllzgngaksqkvyv42lx6y82teum6q3307xw

# TIA<>USDC
CONTRACT_ADDRESS=neutron1wv8pl7tsatzx6n9yaqfksvu5y0x7j50g6mhy636udwfn3vyqp0hsu7g8yk
CONTRACT_ADDRESS=neutron1sj9ax77exvyc86dv30399dw54yehnfajaltj8ugf70vswus9xuks4ldwtu

# ATOM<>USDC
CONTRACT_ADDRESS=neutron1krqwpk0kmphl93kykavp2fnr88g5rnrpk40c34a55yrl00tmfz0s99ewc6
CONTRACT_ADDRESS=neutron1rtvdz9u2zdwtadc8zglawau3f6jrh8jefxjwayl9jc42e24t5rlsflpchs

# Deprecated wBTC<>USDC
CONTRACT_ADDRESS=neutron17fyzkafg4scrd6xu0sp9llrl6hazegza7yer4erlea0kvk30yxsqk2xqfd
CONTRACT_ADDRESS=neutron1me4fuchq3pgle46dvdxsgvpz02z605gkr0sgs6uwew25cpgg3ydsfg8zms
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
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1tvg8hupp64s7aycmhdw638w9g5zkuhc643ywcg9upa0md0kcm4us9eh0kc\", \"treasury\": \"neutron1d9uwvv4w4n8yfnng8enl643vml4xwke63m63gl\", \"strategy_cap\": \"50000000000\", \"float\": \"0.02\",\"token0\": \"ibc/DF8722298D192AAB85D86D0462E8166234A6A9A572DD4A2EA7996029DF4DB363\",\"token1\": \"ibc/B559A80D62249C8AA07A380E2A2BEA6E5CA9A6F079C912C3A9E9B494105E4F81\", \"performance_fee_rate\": \"0.15\", \"management_fee_rate\": \"0.02\", \"vault_type\": \"fund\"}" --label="margined-locust-fund-vault-wbtc.axl" --admin deployer --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID  --amount 1018ibc/DF8722298D192AAB85D86D0462E8166234A6A9A572DD4A2EA7996029DF4DB363
```

### Instantiate - Strategy Drop

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1nz852flh6np9xlg9ju3ka6w5txezsxt0j4lypn\", \"token0\": \"ibc/C4CFF46FD6DE35CA4CF4CE031E643C8FDC9BA4B99AE598E9B0ED98FE3A2319F9\", \"token1\": \"ibc/B559A80D62249C8AA07A380E2A2BEA6E5CA9A6F079C912C3A9E9B494105E4F81\", \"grants\": [\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgWithdrawFilledLimitOrder\",\"/neutron.dex.MsgCancelLimitOrder\"], \"pool_info\": {\"slinky\": {\"timeout\": 900, \"base\": \"ATOM\",\"quote\": \"USD\"}}}" --label="margined-locust-strategy-{$DENOMS}" --admin deployer-neutron --from=deployer-neutron --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Instantiate - Strategy Astroport

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1nz852flh6np9xlg9ju3ka6w5txezsxt0j4lypn\", \"token0\": \"ibc/773B4D0A3CD667B2275D5A4A7A2F0909C0BA0F4059C0B9181E680DDF4965DCC7\", \"grants\": [\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgWithdrawFilledLimitOrder\",\"/neutron.dex.MsgCancelLimitOrder\"], \"pool_info\": {\"astroport\": {\"pool_address\": \"neutron1f6wucml5pmys4uh7mwurz2ge2v7gamkqdy03rfuatpdphmjjag3qeutzgx\", \"token0\": \"ibc/773B4D0A3CD667B2275D5A4A7A2F0909C0BA0F4059C0B9181E680DDF4965DCC7\",\"token1\": \"factory/neutron1ut4c6pv4u6vyu97yw48y8g7mle0cat54848v6m97k977022lzxtsaqsgmq/udtia\"}}}" --label="margined-locust-strategy-dtia" --admin deployer-neutron --from=deployer-neutron --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Instantiate - Strategy Slinky

```bash
neutrond tx wasm instantiate $CODE_ID "{\"admin\": \"neutron1ha2hjlce7sqp59g8xhxz2jds97x8fdw9k9wngp\", \"controller\": \"neutron1nz852flh6np9xlg9ju3ka6w5txezsxt0j4lypn\", \"token0\": \"ibc/DF8722298D192AAB85D86D0462E8166234A6A9A572DD4A2EA7996029DF4DB363\",\"token1\": \"ibc/B559A80D62249C8AA07A380E2A2BEA6E5CA9A6F079C912C3A9E9B494105E4F81\", \"grants\": [\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgWithdrawFilledLimitOrder\",\"/neutron.dex.MsgCancelLimitOrder\"], \"pool_info\": {\"slinky\": {\"base\": \"BTC\", \"quote\": \"USD\",\"timeout\": 900}}}" --label="margined-locust-strategy-wbtc.axl" --admin deployer --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
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

#### Register Sudo

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS  "{\"vault_extension\": {\"vaultenator\": {\"register_sudo\": {}}}}" --from=deployer-neutron --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Set Open

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"set_open\": {}}}}"  --from=deployer-neutron --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Set Vault

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"set_vault\": {\"vault\":\"neutron1egc0ujxyqh8p35nxrvxd04uq0z9536k6fvwzwecfgjj9yg7wkdgq2jzj38\"}}"  --from=deployer --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Update Config - Fund

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"update_config\": {\"new_config\": {\"strategy_cap\": \"500000000000\"}}}}}"  --from=deployer --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"update_config\": {\"new_config\": {\"controller\": \"neutron1ajk4hcvtf48qwt773v8cpwraq2qtj9kum6x6twua8jzv7863y8csqs64ca\"}}}}}"  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

#### Update Config - Strategy

```bash
neutrond tx wasm execute $CONTRACT_ADDRESS "{\"set_grants\": {\"grants\":  [\"/cosmwasm.wasm.v1.MsgExecuteContract\",\"/neutron.dex.MsgPlaceLimitOrder\",\"/neutron.dex.MsgDeposit\",\"/neutron.dex.MsgWithdrawal\"]}}"  --from=deployer-neutron --gas=auto --gas-prices 0.0053untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Migrate

```bash
neutrond tx wasm migrate $CONTRACT_ADDRESS $CODE_ID '{}'  --from=deployer --gas=auto --gas-prices 0.008untrn --gas-adjustment 1.3 --output=json --node=$NODE --chain-id=$CHAIN_ID
```

### Query

#### Config

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"config\": {}}}}" --node=$NODE --output=json | jq .
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"config\": {}}" --node=$NODE --output=json | jq .
```

#### State

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"state\": {}}}}" --node=$NODE --output=json | jq .
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"state\": {}}" --node=$NODE --output=json | jq .
```

#### Estimate Vault Assets

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"estimate_vault_assets\": { \"amount\": \"1000\" }}}}" --node=$NODE --output=json | jq .
```

#### Total Vault Token Supply

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"total_vault_token_supply\": {}}" --node=$NODE --output=json | jq .
```

#### Version

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"version\": {}}}}" --node=$NODE --output=json | jq .
```

#### Withdrawable Amount

```bash
neutrond q wasm contract-state smart $CONTRACT_ADDRESS "{\"vault_extension\": {\"vaultenator\": {\"withdrawable_amount\": {}}}}" --node=$NODE --output=json | jq .
```

#### Wasm Contract Info

```bash
neutrond q wasm contract $CONTRACT_ADDRESS  --output=json --node=$NODE  | jq .
```

#### Tokenfactory Params

```bash
neutrond q tokenfactory params --output=json --node=$NODE  | jq .
```

#### Tokenfactory Check Hooks

```bash
neutrond q tokenfactory before-send-hook factory/$CONTRACT_ADDRESS/fund  --output=json --node=$NODE  | jq .
```

#### Contract Info

```bash
neutrond q wasm contract $CONTRACT_ADDRESS --node=$NODE --output=json | jq .
```

### Other

```bash
neutrond q marketmap market-map --node=$NODE --output json | jq .
neutrond q oracle currency-pairs  --node=$NODE --output json | jq .
```

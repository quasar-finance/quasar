# Token factory demo guide 

This demo guide follow the instructions provided by token factory binding implementation in
https://github.com/CosmosContracts/token-bindings/ 

You can look into that for more information, steps are reproduced here for custom to the quasar chain binary.
## Testing steps with smart contract

### Steps 1 - checkout the token binding codebase
```bash
git clone git@github.com:CosmosContracts/token-bindings.git

```

### Build 
#### Wasm compile 
```bash
cd contracts/tokenfactory
rustup default stable
cargo wasm
```

#### Optimized compilation
Note - Remove the target directory from base directory and inside the tokenfactory directory.

```bash
sudo docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/workspace-optimizer:0.13.0
```
It will generate artifacts directory in the project root. And check if the artifacts 
directory contains the newly build .wasm files.
```bash
cd artifacts
```
 
### Upload the artifacts
```bash
TX=$(quasard tx wasm store tokenfactory.wasm  --from alice --chain-id=quasar --gas-prices 0.1uqsr --keyring-backend test --home ~/.quasarnode --gas auto --gas-adjustment 1.3 -b block --output json -y --node tcp://localhost:26659 | jq -r '.txhash')
CODE_ID=$(quasard query tx $TX  --node tcp://localhost:26659 --output json | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Your contract code_id is $CODE_ID"
```

### Instantiate the contract
```bash

quasard tx wasm instantiate $CODE_ID "{}" --amount 50000uqsr  --label "Token Factory Contract" --from alice --keyring-backend test --home ~/.quasarnode  --chain-id quasar --gas-prices 0.1uqsr --gas auto --gas-adjustment 1.3 -b block -y --no-admin --node tcp://localhost:26659

CONTRACT_ADDR=$(quasard query wasm list-contract-by-code $CODE_ID --node tcp://localhost:26659 --output json | jq -r '.contracts[0]')
echo "Your contract address is $CONTRACT_ADDR"
```

### Execute & Queries

#### Generate schema

```bash
cd contracts/tokenfactory
cargo schema # generates schema in the contracts/tokenfactory/schema folder

```

#### Create denom


```bash
quasard tx wasm execute $CONTRACT_ADDR '{ "create_denom": { "subdenom": "mydenom" } }' --from alice --amount 1000000000uqsr -b block --keyring-backend test --home ~/.quasarnode --chain-id quasar --node tcp://localhost:26659

quasard q bank total --denom factory/$CONTRACT_ADDR/mydenom --node tcp://localhost:26659
# You should see this:
# amount: "0"
#denom: factory/osmo1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqcm3670/mydenom
```
#### mint 
BOB=$(quasard keys show bob --keyring-backend test -a)
```bash
quasard tx wasm execute $CONTRACT_ADDR "{ \"mint_tokens\": {\"amount\": \"100\", \"denom\": \"factory/${CONTRACT_ADDR}/mydenom\", \"mint_to_address\": \"$BOB\"}}" --from alice --keyring-backend test --home ~/.quasarnode --chain-id quasar --node tcp://localhost:26659 -b block
quasard q bank balances $BOB --node tcp://localhost:26659
quasard q bank total --denom factory/$CONTRACT_ADDR/mydenom --node tcp://localhost:26659
```


#### burn ( only contract can burn as of now)
1. Pre-Mint tokens to contract address.
2. Burn from contract address.
```bash

quasard tx wasm execute $CONTRACT_ADDR "{ \"mint_tokens\": {\"amount\": \"100\", \"denom\": \"factory/${CONTRACT_ADDR}/mydenom\", \"mint_to_address\": \"$CONTRACT_ADDR\"}}" --from alice --keyring-backend test --home ~/.quasarnode --chain-id quasar --node tcp://localhost:26659 -b block
quasard tx wasm execute $CONTRACT_ADDR "{ \"burn_tokens\": {\"amount\": \"50\", \"denom\": \"factory/${CONTRACT_ADDR}/mydenom\", \"burn_from_address\": \"\"}}" --from alice --keyring-backend test --home ~/.quasarnode --chain-id quasar --node tcp://localhost:26659 -b block
quasard q bank total --denom factory/$CONTRACT_ADDR/mydenom --node tcp://localhost:26659
quasard q bank balances $CONTRACT_ADDR  --node tcp://localhost:26659
```


### change Admin

```bash
quasard q tokenfactory denom-authority-metadata factory/quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu/mydenom --node tcp://localhost:26659

 quasard tx wasm execute $CONTRACT_ADDR "{ \"change_admin\": {\"denom\": \"factory/${CONTRACT_ADDR}/mydenom\", \"new_admin_address\": \"$BOB\"}}" --from alice --keyring-backend test --home ~/.quasarnode --chain-id quasar --node tcp://localhost:26659 -b block 

```

### Get denom
```bash

quasard query wasm contract-state smart $CONTRACT_ADDR "{ \"get_denom\": {\"creator_address\": \"${CONTRACT_ADDR}\", \"subdenom\": \"mydenom\" }}" --node tcp://localhost:26659
```
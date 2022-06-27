# Quasar vaults
A Quasar vault is a smart contract to receive Cosmos native tokens

The packages directory provides packages for strategists to ease development

## Building the CLI
These smart contracts are build on the Quasar chain. Using ``` go install -mod=readonly ./cmd/quasarnoded/``` from the quasar root directory. we build de quasarnoded CLI 
## Running the chain locally
These smart contracts are build on the Quasar chain, thus we also want to run the chain.
```
ignite chain serve
```
## Building a contract
For setting up prerequisites, refer to `https://docs.cosmwasm.com/docs/1.0/getting-started/installation`
Here we assume that the correct CosmWasm setup has been done.
To check whether we can build the contract, in the directory of the contract, run `cargo wasm`.
To actually build the contract to be uploaded to the chain, we need to build an optimized version using the rust optimizer docker image from CosmWasm.
In the smart contracts directory, run:
```
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/workspace-optimizer:0.12.6
```
this builds all contracts in the contracts directory and places the wasm files in `smart-contracts/artifacts`.
We need to use the workspace optimizer because we have a separate directory where our packages reside.

## Instantiating a contract
Now that we have our CLI, a running chain and our compiled contract, we can now upload the contract and instantiate it.
to upload the contract:
```
quasarnoded tx wasm store ./artifacts/cw_4626.wasm --from alice --gas auto
```
check that it's uploaded:
```
quasarnoded query wasm list-code --node http://0.0.0.0:26657
```

now to instantiate it, we first create the instantaite message and safe that into an env var for later user:
```
INSTANTIATE='{"name":"test-vault","symbol":"TEV","reserve_denom":"uqsar","reserve_total_supply":"100000","reserve_decimals":6,"supply_decimals":6,"initial_balances":[],"mint":null,"marketing":null,"curve_type":{"constant":{"value":"15","scale":1}}}'
```
For easy editing, the instantion of the vault can also be found in `readme_instantion.json`
```
quasarnoded tx wasm instantiate 1 "$INSTANTIATE" --label test --no-admin --from alice
```
## Interacting with the contract

Now lets deposit some funds:
```
EXECUTE='{"deposit":{}}'
```
```
 quasarnoded tx wasm execute YOUR_CONTRACT_ADDRESS "$EXECUTE" --amount 1000uatom --from alice
```


## Where to find what
- The vault contract: cw-4626. This contract calls into a strategy contract for all it's strategy logic
- The Strategy contract: TBD


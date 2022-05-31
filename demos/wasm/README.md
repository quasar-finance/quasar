# WASM demo

This demo tutorial shows deployment of a wasm smart contract.

## Setup
We need to setup the CLI:
in the root directory of the repository run
``` go install -mod=readonly ./cmd/quasarnoded/```
now we can use quasarnoded on the commandline to interact with the wasm module

Now we need an optimized contract to  upload and interact with. With this demo, we bundled the name service contract
First we compile the contract, from `wasm/nameservice` run 
```docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.11.3```

Now that we can interact with the chain and have a contract to work with, we can do the actual demo.

## Demo
Run the local node:
```ignite chain serve --reset-once```

List all wasm bytecodes on chain:
```wasmd query wasm list-code --node http://0.0.0.0:26657```
And we have show of life!

Lets upload a contract, running from the `wasm` dir
``` quasarnoded tx wasm store ./nameservice/artifacts/cw_nameservice.wasm --from alice --gas auto```

rerunning the list command:
```quasarnoded query wasm list-code --node http://0.0.0.0:26657```
We now see our created contract and instantiate it.
```quasarnoded tx wasm instantiate 1 {} --label test --no-admin --from alice```
# Quasar

This is the official Quasar Labs repository.
Quasar is a decentralized app-chain built for interchain asset management.

Quasar is focused in utilizing the latest and contributing to building IBC features including:
IBC features that we are developing on:
1. Interchain Accounts (ICA).
2. Async Interchain Queries (Async - ICQ).
3. IBC hook middleware for token transfer. 

Quasar is working hard to simplfy and add ease to collaborative investment with digital assets. 

We are creating a decentralized platform for creating custom, soverign vaults that can be molded into any imaginable investment financial instrument from ETFs, mutual fund, SPAC, or whatever. 
The galaxy is the limit. 

Our flagship product starts with vault that implements optimal LPing into pools on Osmosis DEX.

## DISCLAIMER
The current codebase is experimental and undergoing continuous testing and auditing. 

## Quasar Node
**quasarnode** is a capital management blockchain built using Cosmos SDK, delegated proof of stake and ibc stack. 

### Build Quasar

```bash
make install 
```

  
## Learn more
1. https://www.quasar.fi/
2. https://app.quasar.fi/
 
## Attributions

x/qtransfer, x/epochs and x/tokenfactory module are utilised from the osmosis x/ibc_hooks, x/epochs and x/tokenfactory module.


## Dependencies
### Rust
In order to run test-tube the following dependencies are required:
* `sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev curl clang libclang-dev`
* go1.21 ([see here](https://go.dev/doc/install))
* libwasmvm ([see here](https://github.com/CosmWasm/wasmvm) -- !Instructions don't cover installation, copy the files to your desired install location or add the subfolder `wasmvm/internal/api` to your library paths) 

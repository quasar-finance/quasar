# Quasar

#### Chain:
![](https://github.com/quasar-finance/quasar/actions/workflows/build_go.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/lint_go.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/test_go.yml/badge.svg)

#### Contracts (rust)
![](https://github.com/quasar-finance/quasar/actions/workflows/cl_vault.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/dex_router_osmosis.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/lst_adapter_osmosis.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/lst_dex_adapter_osmosis.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/merkle_incentives.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/range_middleware.yml/badge.svg)
![](https://github.com/quasar-finance/quasar/actions/workflows/token_burner.yml/badge.svg)

#### Packages (rust)
![](https://github.com/quasar-finance/quasar/actions/workflows/quasar_types.yml/badge.svg)

## Overview

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

In order to speed up compilation times you can try to out [cachepot](https://github.com/paritytech/cachepot):
* install: `cargo install --git https://github.com/paritytech/cachepot`
* use: `export RUSTC_WRAPPER="cachepot"`

## Git hooks
To automatically use both the pre-commit hook and the post-merge hook, you can adjust the hook path of git: `git config core.hooksPath ${PWD}/scripts/git`.

#### Pre-commit hook
Enable the pre-commit hook by copying the entrypoint to the hooks folder: `cp scripts/pre-commit .git/hooks`.

It forwards to `scripts/git/pre-commit`, which contains the actual implementation.
If you are concerned about automatically picking up changes in a bash script from the repository you may install the pre-commit hook via: `cp scripts/git/pre-commit .git/hooks`

#### Post-merge hook
The post-merge hook automatically removes branches that have been removed on the remote.
WARNING: Only use this if you are disciplined with your usage of git.

## IDE Configuration
Rust-analyzer does not pick up optional dependencies (even when specified as required-dependencies for a target). Moreover, it can get confused it it finds too many workspaces.

#### vscode
The following template enables rust-analyzer for test-tube tests and adds the workspace paths.

Template for `./.vscode/settings.json`:
```
{
    "rust-analyzer.linkedProjects": [
        "<REPO_ROOT>/smart-contracts/osmosis/Cargo.toml",
        "<REPO_ROOT>/smart-contracts/quasar/Cargo.toml",
    ],
    "rust-analyzer.cargo.features": [
        "test-tube"
    ]
}
```

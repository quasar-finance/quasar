# Upgrading Quasar to v3

This guide provides instructions for upgrading to specific versions of Quasar.

## [v0.3.x](https://github.com/quasar-finance/quasar/releases/tag/v3.0.0)

### `Binary Name Change`
- This version contains binary name change from `quasarnoded` to `quasard` which requires changes in Cosmovisor settings.

### `Feemarket Module Addition and Relayer Config Changes`

- It contains Feemarket module addition with min gas fee.
This needs changes on relayer `gas-prices` to be set equal to params set in feemarket module.

- Feemarket brings in variable gas depending on past few block usage which in turn will affect the `gas-adjustment` parameter as well. 
It should ideally be set to 2 or above.

### Packages

New modules added :
- PFM v8
- Ibc-hooks v8
- Rate-limiting v8
- Feemarket v1

Updates : 
- Wasmd v0.45 to v0.51
- Wasmvm from v1 to v2
- Cosmos-SDK from v47.12 to v50.9
- Ibc-go v7 to ibc-go v8
- Contains updates of Async ICQ, wasm light clients and capability module.

Upgrade removes old unused modules : 
- `qVesting`
- `qOracle`
- `qTransfer`


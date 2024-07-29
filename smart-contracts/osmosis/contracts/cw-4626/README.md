# CW-4626 - CW20 Base

Built on the [CW20 Base](https://github.com/CosmWasm/cw-plus/tree/main/contracts/cw20-base) contract, this contract 
is a basic implementation of [eip-4626](https://eips.ethereum.org/EIPS/eip-4626).
This is a basic implementation of a cw20 contract. It implements
the [CW20 spec](quasar-finance/quasar/smart-contracts/cw-plus/packages/cw20/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw20-compatible tokens with custom logic.

Implements:
    
- [x] CW20 Base
- [x] Mintable extension
- [x] Allowances extension
- [ ] EIP-4626 implementation
  - [ ] Change CW-20 asset to vault share
  - [ ] implement EIP-4626 interface for cosmwasm
    - [x] Implement token whitelist
    - [ ] Add share minting logic
    - [ ] Add share burning logic
    - [ ] Add support for all queries
    - [ ] Add support for all execute messages
- [ ] Multi-Asset Deposit
- [ ] Multi-Asset Withdrawal
- [ ] Strategy trait

## Running this contract

You will need Rust 1.44.1+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via:

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw20_base.wasm .
ls -l cw20_base.wasm
sha256sum cw20_base.wasm
```

Or for a production-ready (optimized) build, run a build command in the
the repository root: https://github.com/CosmWasm/cw-plus#compiling.

## Importing this contract

You can also import much of the logic of this contract to build another
ERC20-contract, such as a bonding curve, overiding or extending what you
need.

Basically, you just need to write your handle function and import
`cw20_base::contract::handle_transfer`, etc and dispatch to them.
This allows you to use custom `ExecuteMsg` and `QueryMsg` with your additional
calls, but then use the underlying implementation for the standard cw20
messages you want to support. The same with `QueryMsg`. You *could* reuse `instantiate`
as it, but it is likely you will want to change it. And it is rather simple.

Look at [`cw20-staking`](https://github.com/CosmWasm/cw-tokens/tree/main/contracts/cw20-staking) for an example of how to "inherit"
all this token functionality and combine it with custom logic.

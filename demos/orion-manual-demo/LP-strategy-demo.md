# LP-strategy demo

## Introduction
The LP strategy demo shows how to test the current version of the LP-strategy. This current version only executes the actual strategy part of the LP-strategy, meaning sending tokens to osmosis, joining a pool through join-swap extern-amount-in and locking the tokens.

## setup
To get this demo working, at the minimum a local [osmosis](https://github.com/osmosis-labs/osmosis) release >= v13 such as the [13.1.2](https://github.com/osmosis-labs/osmosis/releases/tag/v13.1.2) release, the local quasard contained in this repo and the [go relayer](https://github.com/cosmos/relayer/releases/tag/v2.1.2). Optionally, if packet forwarding needs to be tested aswell, a local chain of that token is also needed. Most logical is to try this with [gaiad](https://github.com/cosmos/gaia/releases/tag/v7.1.0) for uatom.

## execution
the `run_all.sh` script sets up the local chains, and starts to run the go relayer. The current setup of the go relayer assumes that a local gaiad instance is running. Through `create_and_execute_contract.sh`, a contract is setup and a channel between the contract and the ica host on osmosis is made. The contract is now ready to be interacted with. 

## what's where
The contract currently provides 2 entry points: `TransferJoinLock` and `DepositAndLockTokens`; `TransferJoinLock` is the entrypoint as expected to be used and called by the bookkeeping vault. `DepositAndLockTokens` allows for more parameters to be set by the caller. This message does not trigger a transfer. Eventually this will be removed and replaced with better testing setups.

Internally, after one of the two initial execute messages is called, an ibc packet is dispatched, the sequence number is saved and an enum is saved. On the ibc ack, the original message's enum is looked up in the contracts state and a new ibc message is dispatched. The message are done in the following order. ibc_transfer -> join_pool_extern_swap_amount_in -> lock_tokens. ibc acks to transfer messages are provided by our local transfer decorator, found in `decorators/ibc_transfer_wasm_decorator.go`
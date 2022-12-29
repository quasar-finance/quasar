# LP-strategy

## current state
Currently, the strategy supports receiving funds from a depositor, transferring those funds over a channel to it's own ICA address, joining a predetermined pool with those funds and locking those funds for two weeks

## to be added
The strategy currently lacks the following that need to be added
- [ ] bookkeeping
- [ ] withdraws (dependant on ibc hooks on quasar, memo field in ibc transfer over ICA)
- [ ] move channel logic to quasar-types 

## how does the contract work
In ibc.rs, we have the cosmwasm entrypoints to setup the different IBC connections, this contract currently supports ICQ and ICA channels, but there is currently no use case for ICQ.

After channels are setup, users can call the `TransferJoinLock` and `DepositAndLockTokens` execute Msgs, `TransferJoinLock`, transfers funds, joins the pool and locks the tokens, `DepositAndLockTokens` only joins the pool and locks the tokens

## Testing locally
In demos/orion-manual-demo, `run_all.sh` sets up all chain needed, and `create_and_execute_contract` sets up a contract and starts trying to `TransferJoinLock` for Alice
# Lst adapter between osmosis and stride

There are three parties that can perform action on this vault:
 * Owner: can change the configuration
 * Vault: can trigger unbonding of LSTs and claim unbonded tokens
 * Observer: can confirm the initiation of an unbonding process on stride as well as the receival of unbonded tokens on osmosis 

# How it works

We can't query outstanding redemptions on stride on-chain. Thus, we need to monitor them off-chain. Similarly, when we receive the underlying token, these just occur in the lst adapter at some point in time (stride IBC-transfers them back when they unlock).
Therefore, we need an off-chain observer that informs the lst-adapter when
 * an unbonding process, that was initiated from osmosis through an IBC-transfer and stride's autopilot, is actually started on stride (param: the exact amount of tokens that we will receive when unbonding finishes)
 * when an unbonding finishes (param: unbonding start timestamp)

### Unbond
In practice this means the following process for a fully confirmed unbonding:
* trigger unbond (vault): `LstAdapterExecuteMsg::Unbond`
  * this either triggers IBC-transfer and unbonding process via autopilot
  * or, if an unbonding process is yet unconfirmed, keeps the LST tokens in the lst-adapter until the next unbonding process is confirmed and the next unbond is executed
* confirm unbond as soon as it gets observed on stride (observer): `LstAdapterExecutMsg::ConfirmUnbond`

The possibly delay in unbonding until the last unbond gets confirmed can be loosened. Until this is fully tested and stable, I prefer to keep things simple at the risk of slightly delaying the unbonding process. This is not nice, but not a big issue as the delays should be negligible wrt. the unbonding period. 

One strategy for loosening this restriction is to take into account the unbond amount and the max change of the redemption rate until an IBC-timeout occurs. This would allow define a narrow range in which the confirmation must fall, so IBC-transactions for unbondings that have non-overlapping ranges could be processed asynchronously. 
### Claim
Assuming no claiming of "donations", the claim process looks as follows:
* confirm reception of tokens from stride (observer): `LstAdapterExecuteMsg::ConfirmUnbondFinished`
* claim (vault): `LstAdapterExecuteMsg::Claim`

# Internals:
In order to track the tokens that correspond to the lst-adapter, two internal variables are used
 * TOTAL_BALANCE: tracks the total balance in terms of the underlying token
 * REDEEMED_BALANCE: tracks the amount of unclaimed and redeemed tokens
 
These variables allow us to track funds, without interference through "donations". In order to keep the complexity of the interference of "donations" low, these are added to the lst-vault balance at two places:
 * "donations" in terms of the underlying token are accounted for during the claim process, (i.e. the calling contract should call Claim in a SubMsg in order to correctly parse the received amount either from events or through a query)
 * "donations" in terms of the lst-token are accounted for when an LST-transfer to stride is initiated
 "Donations" in other tokens are ignored.

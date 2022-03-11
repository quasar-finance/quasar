# Messages

## MsgSendIbcCreatePool
Send a message to intergamm module of osmosis side (on the specified channel with `port` and `channelID`) to create an osmosis balancer pool with the given params. This message executes the `MsgCreateBalancerPool` of `gamm` module, `poolParams`, `poolAssets` and `future_pool_governor` are as described in osmosis documents.

## MsgSendIbcJoinPool
Send a message to intergamm module of osmosis side (on the specified channel with `port` and `channelID`) to join a pool with the given params. This message executes the `MsgJoinPool` of `gamm` module, `poolId`, `shareOutAmount` and `tokenInMaxs` are as described in osmosis documents. 

## MsgSendIbcExitPool
Send a message to intergamm module of osmosis side (on the specified channel with `port` and `channelID`) to exit funds from a pool with the given params. This message executes the `MsgExitPool` of `gamm` module, `poolId`, `shareInAmount` and `tokenOutMins` are as described in osmosis documents.

## MsgSendIbcWithdraw
Request an ibc transfer of the specified amount of tokens from intergamm channel address to an address on another chain. Destination can be specified with `transferPort`, `transferChannel` and `receiver` address. (Note that `transferPort` and `transferChannel` should be configured according to osmosis environment)
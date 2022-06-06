<!--
order: 3
-->

# Keepers

## Keeper functions

The Intergamm keeper exposes functions to directly talk to the remote chain.

```go
type IntergammKeeper interface {
	RegisterInterchainAccount(ctx sdk.Context, connectionID, owner string) error

	TransmitIbcCreatePool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolParams *gammbalancer.BalancerPoolParams,
		poolAssets []gammtypes.PoolAsset,
		futurePoolGovernor string,
	) error

	TransmitIbcJoinPool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		shareOutAmount sdk.Int,
		tokenInMaxs []sdk.Coin,
	) error

	TransmitIbcExitPool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		shareInAmount sdk.Int,
		tokenOutMins []sdk.Coin,
	) error

	TransmitIbcJoinSwapExternAmountIn(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		tokenIn sdk.Coin,
		shareOutMinAmount sdk.Int,
	) (uint64, error)

	TransmitIbcExitSwapExternAmountOut(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		tokenOut sdk.Coin,
		shareInMaxAmount sdk.Int,
	) (uint64, error)

	TransmitIbcJoinSwapShareAmountOut(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		tokenInDenom string,
		shareOutAmount sdk.Int,
		tokenInMaxAmount sdk.Int,
	) (uint64, error)

	TransmitIbcExitSwapShareAmountIn(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		tokenOutDenom string,
		shareInAmount sdk.Int,
		tokenOutMinAmount sdk.Int,
	) (uint64, error)

	TransmitIbcLockTokens(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		duration time.Duration,
		coins sdk.Coins,
	) (uint64, error)

	TransmitIbcTransfer(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		transferPort, transferChannel string,
		token sdk.Coin,
		receiver string,
		transferTimeoutHeight ibcclienttypes.Height,
		transferTimeoutTimestamp uint64,
	) error
}
```

These functions are to be consumed by other modules.

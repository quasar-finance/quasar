<!--
order: 3
-->

# Keepers

## Transactions

Intergamm allows for a couple of transactions, namely the ones smart contract strategies need to functions. The transactions are:
### TransmitIbcJoinPool
```go
type MsgTransmitIbcJoinPool struct {
    Creator          string       `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
    ConnectionId     string       `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
    TimeoutTimestamp uint64       `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
    PoolId           uint64       `protobuf:"varint,4,opt,name=poolId,proto3" json:"poolId,omitempty"`
    ShareOutAmount   int64        `protobuf:"varint,5,opt,name=shareOutAmount,proto3" json:"shareOutAmount,omitempty"`
    TokenInMaxs      []types.Coin `protobuf:"bytes,6,rep,name=tokenInMaxs,proto3" json:"tokenInMaxs"`
}
```
### TransmitIbcExitPool
```go
type MsgTransmitIbcExitPool struct {
	Creator          string       `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	ConnectionId     string       `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
	TimeoutTimestamp uint64       `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
	PoolId           uint64       `protobuf:"varint,4,opt,name=poolId,proto3" json:"poolId,omitempty"`
	ShareInAmount    int64        `protobuf:"varint,5,opt,name=shareInAmount,proto3" json:"shareInAmount,omitempty"`
	TokenOutMins     []types.Coin `protobuf:"bytes,6,rep,name=tokenOutMins,proto3" json:"tokenOutMins"`
}
```
### TransmitIbcLockTokens
```go
type MsgTransmitIbcLockTokens struct {
	Creator          string               `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	ConnectionId     string               `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
	TimeoutTimestamp uint64               `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
	Duration         *durationpb.Duration `protobuf:"bytes,4,opt,name=duration,proto3" json:"duration,omitempty"`
	Coins            []types.Coin         `protobuf:"bytes,5,rep,name=coins,proto3" json:"coins"`
}
```

### TransmitIbcBeginUnlocking

```go
type MsgTransmitIbcBeginUnlocking struct {
	Creator          string       `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	ConnectionId     string       `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
	TimeoutTimestamp uint64       `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
	Id               uint64       `protobuf:"varint,4,opt,name=id,proto3" json:"id,omitempty"`
	Coins            []types.Coin `protobuf:"bytes,5,rep,name=coins,proto3" json:"coins"`
}
```

### TransmitIbcJoinSwapExternAmountIn

```go
type MsgTransmitIbcJoinSwapExternAmountIn struct {
	Creator           string     `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
	ConnectionId      string     `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
	TimeoutTimestamp  uint64     `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
	PoolId            uint64     `protobuf:"varint,4,opt,name=poolId,proto3" json:"poolId,omitempty"`
	ShareOutMinAmount int64      `protobuf:"varint,5,opt,name=shareOutMinAmount,proto3" json:"shareOutMinAmount,omitempty"`
	TokenIn           types.Coin `protobuf:"bytes,6,opt,name=tokenIn,proto3" json:"tokenIn"`
}
```


### TransmitIbcExitSwapExternAmountOut
```go
type MsgTransmitIbcExitSwapExternAmountOut struct {
Creator          string     `protobuf:"bytes,1,opt,name=creator,proto3" json:"creator,omitempty"`
ConnectionId     string     `protobuf:"bytes,2,opt,name=connectionId,proto3" json:"connectionId,omitempty"`
TimeoutTimestamp uint64     `protobuf:"varint,3,opt,name=timeoutTimestamp,proto3" json:"timeoutTimestamp,omitempty"`
PoolId           uint64     `protobuf:"varint,4,opt,name=poolId,proto3" json:"poolId,omitempty"`
ShareInAmount    int64      `protobuf:"varint,5,opt,name=shareInAmount,proto3" json:"shareInAmount,omitempty"`
TokenOutMins     types.Coin `protobuf:"bytes,6,opt,name=tokenOutMins,proto3" json:"tokenOutMins"`
}
```

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

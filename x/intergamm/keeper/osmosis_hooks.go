package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	lockuptypes "github.com/quasarlabs/quasarnode/osmosis/lockup/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

type OsmosisHooks struct {
	ackMsgCreateBalancerPool      []func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) error
	ackMsgJoinPool                []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) error
	ackMsgExitPool                []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) error
	ackMsgJoinSwapExternAmountIn  []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]) error
	ackMsgExitSwapExternAmountOut []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]) error
	ackMsgJoinSwapShareAmountOut  []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]) error
	ackMsgExitSwapShareAmountIn   []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]) error
	ackMsgLockTokens              []func(sdk.Context, types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) error
	ackMsgBeginUnlocking          []func(sdk.Context, types.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse]) error

	timeoutMsgCreateBalancerPool      []func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) error
	timeoutMsgJoinPool                []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool]) error
	timeoutMsgExitPool                []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool]) error
	timeoutMsgJoinSwapExternAmountIn  []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]) error
	timeoutMsgExitSwapExternAmountOut []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]) error
	timeoutMsgJoinSwapShareAmountOut  []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]) error
	timeoutMsgExitSwapShareAmountIn   []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]) error
	timeoutMsgLockTokens              []func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgLockTokens]) error
	timeoutMsgBeginUnlocking          []func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgBeginUnlocking]) error
}

func (oh *OsmosisHooks) ClearAckHooks() {
	oh.ackMsgCreateBalancerPool = nil
	oh.ackMsgJoinPool = nil
	oh.ackMsgExitPool = nil
	oh.ackMsgJoinSwapExternAmountIn = nil
	oh.ackMsgExitSwapExternAmountOut = nil
	oh.ackMsgJoinSwapShareAmountOut = nil
	oh.ackMsgExitSwapShareAmountIn = nil
	oh.ackMsgLockTokens = nil
	oh.ackMsgBeginUnlocking = nil
}

func (oh *OsmosisHooks) ClearTimeoutHooks() {
	oh.timeoutMsgCreateBalancerPool = nil
	oh.timeoutMsgJoinPool = nil
	oh.timeoutMsgExitPool = nil
	oh.timeoutMsgJoinSwapExternAmountIn = nil
	oh.timeoutMsgExitSwapExternAmountOut = nil
	oh.timeoutMsgJoinSwapShareAmountOut = nil
	oh.timeoutMsgExitSwapShareAmountIn = nil
	oh.timeoutMsgLockTokens = nil
	oh.timeoutMsgBeginUnlocking = nil
}

func (oh *OsmosisHooks) AddHooksAckMsgCreateBalancerPool(hs ...func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) error) {
	oh.ackMsgCreateBalancerPool = append(oh.ackMsgCreateBalancerPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgJoinPool(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) error) {
	oh.ackMsgJoinPool = append(oh.ackMsgJoinPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgExitPool(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) error) {
	oh.ackMsgExitPool = append(oh.ackMsgExitPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgJoinSwapExternAmountIn(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]) error) {
	oh.ackMsgJoinSwapExternAmountIn = append(oh.ackMsgJoinSwapExternAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgExitSwapExternAmountOut(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]) error) {
	oh.ackMsgExitSwapExternAmountOut = append(oh.ackMsgExitSwapExternAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgJoinSwapShareAmountOut(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]) error) {
	oh.ackMsgJoinSwapShareAmountOut = append(oh.ackMsgJoinSwapShareAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgExitSwapShareAmountIn(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]) error) {
	oh.ackMsgExitSwapShareAmountIn = append(oh.ackMsgExitSwapShareAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgLockTokens(hs ...func(sdk.Context, types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) error) {
	oh.ackMsgLockTokens = append(oh.ackMsgLockTokens, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgBeginUnlocking(hs ...func(sdk.Context, types.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse]) error) {
	oh.ackMsgBeginUnlocking = append(oh.ackMsgBeginUnlocking, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgCreateBalancerPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) error) {
	oh.timeoutMsgCreateBalancerPool = append(oh.timeoutMsgCreateBalancerPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool]) error) {
	oh.timeoutMsgJoinPool = append(oh.timeoutMsgJoinPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool]) error) {
	oh.timeoutMsgExitPool = append(oh.timeoutMsgExitPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinSwapExternAmountIn(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]) error) {
	oh.timeoutMsgJoinSwapExternAmountIn = append(oh.timeoutMsgJoinSwapExternAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitSwapExternAmountOut(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]) error) {
	oh.timeoutMsgExitSwapExternAmountOut = append(oh.timeoutMsgExitSwapExternAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinSwapShareAmountOut(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]) error) {
	oh.timeoutMsgJoinSwapShareAmountOut = append(oh.timeoutMsgJoinSwapShareAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitSwapShareAmountIn(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]) error) {
	oh.timeoutMsgExitSwapShareAmountIn = append(oh.timeoutMsgExitSwapShareAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgLockTokens(hs ...func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgLockTokens]) error) {
	oh.timeoutMsgLockTokens = append(oh.timeoutMsgLockTokens, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgBeginUnlocking(hs ...func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgBeginUnlocking]) error) {
	oh.timeoutMsgBeginUnlocking = append(oh.timeoutMsgBeginUnlocking, hs...)
}

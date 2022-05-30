package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	lockuptypes "github.com/osmosis-labs/osmosis/v7/x/lockup/types"
)

type IbcTransferHooks struct {
	ackIbcTransfer     []func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse])
	timeoutIbcTransfer []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData])
}

type OsmosisHooks struct {
	ackMsgCreateBalancerPool      []func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])
	ackMsgJoinPool                []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])
	ackMsgExitPool                []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])
	ackMsgJoinSwapExternAmountIn  []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse])
	ackMsgExitSwapExternAmountOut []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse])
	ackMsgLockTokens              []func(sdk.Context, types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse])

	timeoutMsgCreateBalancerPool      []func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])
	timeoutMsgJoinPool                []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])
	timeoutMsgExitPool                []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])
	timeoutMsgJoinSwapExternAmountIn  []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn])
	timeoutMsgExitSwapExternAmountOut []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut])
	timeoutMsgLockTokens              []func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgLockTokens])
}

func (ih *IbcTransferHooks) ClearAckHooks() {
	ih.ackIbcTransfer = nil
}

func (ih *IbcTransferHooks) ClearTimeoutHooks() {
	ih.timeoutIbcTransfer = nil
}

func (ih *OsmosisHooks) ClearAckHooks() {
	ih.ackMsgCreateBalancerPool = nil
	ih.ackMsgJoinPool = nil
	ih.ackMsgExitPool = nil
	ih.ackMsgJoinSwapExternAmountIn = nil
	ih.ackMsgExitSwapExternAmountOut = nil
	ih.ackMsgLockTokens = nil
}

func (ih *OsmosisHooks) ClearTimeoutHooks() {
	ih.timeoutMsgCreateBalancerPool = nil
	ih.timeoutMsgJoinPool = nil
	ih.timeoutMsgExitPool = nil
	ih.timeoutMsgJoinSwapExternAmountIn = nil
	ih.timeoutMsgExitSwapExternAmountOut = nil
	ih.timeoutMsgLockTokens = nil
}

func (ih *IbcTransferHooks) AddHooksAckIbcTransfer(hs ...func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse])) {
	ih.ackIbcTransfer = append(ih.ackIbcTransfer, hs...)
}

func (ih *IbcTransferHooks) AddHooksTimeoutIbcTransfer(hs ...func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData])) {
	ih.timeoutIbcTransfer = append(ih.timeoutIbcTransfer, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgCreateBalancerPool(hs ...func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])) {
	oh.ackMsgCreateBalancerPool = append(oh.ackMsgCreateBalancerPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgJoinPool(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])) {
	oh.ackMsgJoinPool = append(oh.ackMsgJoinPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgExitPool(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])) {
	oh.ackMsgExitPool = append(oh.ackMsgExitPool, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgJoinSwapExternAmountIn(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse])) {
	oh.ackMsgJoinSwapExternAmountIn = append(oh.ackMsgJoinSwapExternAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgExitSwapExternAmountOut(hs ...func(sdk.Context, types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse])) {
	oh.ackMsgExitSwapExternAmountOut = append(oh.ackMsgExitSwapExternAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksAckMsgLockTokens(hs ...func(sdk.Context, types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse])) {
	oh.ackMsgLockTokens = append(oh.ackMsgLockTokens, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgCreateBalancerPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])) {
	oh.timeoutMsgCreateBalancerPool = append(oh.timeoutMsgCreateBalancerPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])) {
	oh.timeoutMsgJoinPool = append(oh.timeoutMsgJoinPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])) {
	oh.timeoutMsgExitPool = append(oh.timeoutMsgExitPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinSwapExternAmountIn(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn])) {
	oh.timeoutMsgJoinSwapExternAmountIn = append(oh.timeoutMsgJoinSwapExternAmountIn, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitSwapExternAmountOut(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut])) {
	oh.timeoutMsgExitSwapExternAmountOut = append(oh.timeoutMsgExitSwapExternAmountOut, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgLockTokens(hs ...func(sdk.Context, types.TimeoutExchange[*lockuptypes.MsgLockTokens])) {
	oh.timeoutMsgLockTokens = append(oh.timeoutMsgLockTokens, hs...)
}

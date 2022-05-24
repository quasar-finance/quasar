package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

type IbcHooks struct {
	ackIbcTransfer     []func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse])
	timeoutIbcTransfer []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData])
}

type OsmosisHooks struct {
	ackMsgCreateBalancerPool []func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])
	ackMsgJoinPool           []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])
	ackMsgExitPool           []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])

	timeoutMsgCreateBalancerPool []func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])
	timeoutMsgJoinPool           []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])
	timeoutMsgExitPool           []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])
}

func (ih *IbcHooks) ClearAckHooks() {
	ih.ackIbcTransfer = nil
}

func (ih *IbcHooks) ClearTimeoutHooks() {
	ih.timeoutIbcTransfer = nil
}

func (ih *OsmosisHooks) ClearAckHooks() {
	ih.ackMsgCreateBalancerPool = nil
	ih.ackMsgJoinPool = nil
	ih.ackMsgExitPool = nil
}

func (ih *OsmosisHooks) ClearTimeoutHooks() {
	ih.timeoutMsgCreateBalancerPool = nil
	ih.timeoutMsgJoinPool = nil
	ih.timeoutMsgExitPool = nil
}

func (ih *IbcHooks) AddHooksAckIbcTransfer(hs ...func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse])) {
	ih.ackIbcTransfer = append(ih.ackIbcTransfer, hs...)
}

func (ih *IbcHooks) AddHooksTimeoutIbcTransfer(hs ...func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData])) {
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

func (oh *OsmosisHooks) AddHooksTimeoutMsgCreateBalancerPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])) {
	oh.timeoutMsgCreateBalancerPool = append(oh.timeoutMsgCreateBalancerPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgJoinPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])) {
	oh.timeoutMsgJoinPool = append(oh.timeoutMsgJoinPool, hs...)
}

func (oh *OsmosisHooks) AddHooksTimeoutMsgExitPool(hs ...func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])) {
	oh.timeoutMsgExitPool = append(oh.timeoutMsgExitPool, hs...)
}

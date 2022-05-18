package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

type OsmosisHooks struct {
	ackMsgTransfer           []func(sdk.Context, types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse])
	ackMsgCreateBalancerPool []func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])
	ackMsgJoinPool           []func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])
	ackMsgExitPool           []func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])

	timeoutMsgTransfer           []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.MsgTransfer])
	timeoutMsgCreateBalancerPool []func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])
	timeoutMsgJoinPool           []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])
	timeoutMsgExitPool           []func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])
}

func (oh *OsmosisHooks) AddHookAckMsgTransfer(h func(sdk.Context, types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse])) {
	oh.ackMsgTransfer = append(oh.ackMsgTransfer, h)
}

func (oh *OsmosisHooks) AddHookAckMsgCreateBalancerPool(h func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])) {
	oh.ackMsgCreateBalancerPool = append(oh.ackMsgCreateBalancerPool, h)
}

func (oh *OsmosisHooks) AddHookAckMsgJoinPool(h func(sdk.Context, types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])) {
	oh.ackMsgJoinPool = append(oh.ackMsgJoinPool, h)
}

func (oh *OsmosisHooks) AddHookAckMsgExitPool(h func(sdk.Context, types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])) {
	oh.ackMsgExitPool = append(oh.ackMsgExitPool, h)
}

func (oh *OsmosisHooks) AddHookTimeoutMsgTransfer(h func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.MsgTransfer])) {
	oh.timeoutMsgTransfer = append(oh.timeoutMsgTransfer, h)
}

func (oh *OsmosisHooks) AddHookTimeoutMsgCreateBalancerPool(h func(sdk.Context, types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool])) {
	oh.timeoutMsgCreateBalancerPool = append(oh.timeoutMsgCreateBalancerPool, h)
}

func (oh *OsmosisHooks) AddHookTimeoutMsgJoinPool(h func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgJoinPool])) {
	oh.timeoutMsgJoinPool = append(oh.timeoutMsgJoinPool, h)
}

func (oh *OsmosisHooks) AddHookTimeoutMsgExitPool(h func(sdk.Context, types.TimeoutExchange[*gammtypes.MsgExitPool])) {
	oh.timeoutMsgExitPool = append(oh.timeoutMsgExitPool, h)
}

package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

type OsmosisHooks struct {
	hooksMsgTransfer           []func(sdk.Context, types.Exchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse])
	hooksMsgCreateBalancerPool []func(sdk.Context, types.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])
	hooksMsgJoinPool           []func(sdk.Context, types.Exchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])
	hooksMsgExitPool           []func(sdk.Context, types.Exchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])
}

func (oh *OsmosisHooks) AddHookMsgTransfer(h func(sdk.Context, types.Exchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse])) {
	oh.hooksMsgTransfer = append(oh.hooksMsgTransfer, h)
}

func (oh *OsmosisHooks) AddHookMsgCreateBalancerPool(h func(sdk.Context, types.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse])) {
	oh.hooksMsgCreateBalancerPool = append(oh.hooksMsgCreateBalancerPool, h)
}

func (oh *OsmosisHooks) AddHookMsgJoinPool(h func(sdk.Context, types.Exchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse])) {
	oh.hooksMsgJoinPool = append(oh.hooksMsgJoinPool, h)
}

func (oh *OsmosisHooks) AddHookMsgExitPool(h func(sdk.Context, types.Exchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse])) {
	oh.hooksMsgExitPool = append(oh.hooksMsgExitPool, h)
}

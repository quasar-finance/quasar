package keeper

import (
	intergammtypes "github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

func (k Keeper) HandleMsgTransfer(
	ctx sdk.Context,
	ex intergammtypes.Exchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse],
) {
	k.Logger(ctx).Info("HandleMsgTransfer hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.Exchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) {
	k.Logger(ctx).Info("HandleMsgCreateBalancerPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.Exchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse],
) {
	k.Logger(ctx).Info("HandleMsgJoinPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.Exchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse],
) {
	k.Logger(ctx).Info("HandleMsgExitPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

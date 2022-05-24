package keeper

import (
	intergammtypes "github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

// Intergamm callbacks

// IBC

func (k Keeper) HandleAckIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *intergammtypes.MsgEmptyIbcResponse],
) {
	k.Logger(ctx).Info("HandleAckIbcTransfer hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleTimeoutIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData],
) {
	k.Logger(ctx).Info("HandleTimeoutIbcTransfer hook called", "seq", ex.Sequence)
}

// ICA Osmosis

func (k Keeper) HandleAckMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) {
	k.Logger(ctx).Info("HandleAckMsgCreateBalancerPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleAckMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse],
) {
	k.Logger(ctx).Info("HandleAckMsgJoinPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleAckMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse],
) {
	k.Logger(ctx).Info("HandleAckMsgExitPool hook called", "error", ex.Error, "seq", ex.Sequence)
}

func (k Keeper) HandleTimeoutMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool],
) {
	k.Logger(ctx).Info("HandleTimeoutMsgCreateBalancerPool hook called", "seq", ex.Sequence)
}

func (k Keeper) HandleTimeoutMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgJoinPool],
) {
	k.Logger(ctx).Info("HandleTimeoutMsgJoinPool hook called", "seq", ex.Sequence)
}

func (k Keeper) HandleTimeoutMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgExitPool],
) {
	k.Logger(ctx).Info("HandleTimeoutMsgExitPool hook called", "seq", ex.Sequence)
}

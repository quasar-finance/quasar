package keeper

import (
	intergammtypes "github.com/abag/quasarnode/x/intergamm/types"
	gammtypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm"
	gammbalancer "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
	lockuptypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/lockup"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
)

// Intergamm callbacks

// IBC

func (k Keeper) HandleAckIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *intergammtypes.MsgEmptyIbcResponse],
) error {
	k.Logger(ctx).Info("HandleAckIbcTransfer hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckIcaIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse],
) error {
	k.Logger(ctx).Info("HandleAckIcaIbcTransfer hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData],
) error {
	k.Logger(ctx).Info("HandleTimeoutIbcTransfer hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutIcaIbcTransfer(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*ibctransfertypes.MsgTransfer],
) error {
	k.Logger(ctx).Info("HandleTimeoutIcaIbcTransfer hook called", "seq", ex.Sequence)
	return nil
}

// ICA Osmosis

func (k Keeper) HandleAckMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgCreateBalancerPool hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgJoinPool hook called", "error", ex.Error, "seq", ex.Sequence)
	var err error
	if ex.HasError() {
		err = sdkerrors.Wrapf(types.ErrIcaMessageFailedInHost, ex.Error)
	}
	k.OnJoinPoolAck(ctx, ex.Sequence, err)
	return err
}

func (k Keeper) HandleAckMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgExitPool hook called", "error", ex.Error, "seq", ex.Sequence)
	var err error
	if ex.HasError() {
		err = sdkerrors.Wrapf(types.ErrIcaMessageFailedInHost, ex.Error)
	}
	err = k.OnExitPoolAck(ctx, ex.Sequence, err)
	return err
}

func (k Keeper) HandleAckMsgJoinSwapExternAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgJoinSwapExternAmountIn hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgExitSwapExternAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgExitSwapExternAmountOut hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgJoinSwapShareAmountOut(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgJoinSwapShareAmountOut hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgExitSwapShareAmountIn(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgExitSwapShareAmountIn hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgLockTokens(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgLockTokens hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleAckMsgBeginUnlocking(
	ctx sdk.Context,
	ex intergammtypes.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse],
) error {
	k.Logger(ctx).Info("HandleAckMsgBeginUnlocking hook called", "error", ex.Error, "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgCreateBalancerPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgCreateBalancerPool hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgJoinPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgJoinPool],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgJoinPool hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgExitPool(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgExitPool],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgExitPool hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgJoinSwapExternAmountIn(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgJoinSwapExternAmountIn hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgExitSwapExternAmountOut(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgExitSwapExternAmountOut hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgJoinSwapShareAmountOut(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgJoinSwapShareAmountOut hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgExitSwapShareAmountIn(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgExitSwapShareAmountIn hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgLockTokens(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*lockuptypes.MsgLockTokens],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgLockTokens hook called", "seq", ex.Sequence)
	return nil
}

func (k Keeper) HandleTimeoutMsgBeginUnlocking(
	ctx sdk.Context,
	ex intergammtypes.TimeoutExchange[*lockuptypes.MsgBeginUnlocking],
) error {
	k.Logger(ctx).Info("HandleTimeoutMsgBeginUnlocking hook called", "seq", ex.Sequence)
	return nil
}

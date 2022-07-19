package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (k msgServer) UpdateOsmosisParams(goCtx context.Context, msg *types.MsgUpdateOsmosisParams) (*types.MsgUpdateOsmosisParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Check authority of the sender

	state := k.GetOsmosisParamsRequestState(ctx)
	if state.Pending() {
		return nil, sdkerrors.Wrapf(types.ErrPendingRequest, "tried to send a packet to update osmosis params but another request is pending with sequence %s", state.PacketSequence)
	}

	seq, err := k.sendOsmosisParamsRequest(ctx)
	if err != nil {
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOsmosisParamsRequest,
				sdk.NewAttribute(types.AttributeError, err.Error()),
			))
		return nil, err
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeOsmosisParamsRequest,
			sdk.NewAttribute(types.AtributePacketSequence, fmt.Sprintf("%d", seq)),
		))

	return &types.MsgUpdateOsmosisParamsResponse{PacketSequence: seq}, nil
}

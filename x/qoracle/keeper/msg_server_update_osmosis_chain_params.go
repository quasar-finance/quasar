package keeper

import (
	"context"
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (k msgServer) UpdateOsmosisChainParams(goCtx context.Context, msg *types.MsgUpdateOsmosisChainParams) (*types.MsgUpdateOsmosisChainParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Check authority of the sender

	state := k.GetOsmosisParamsRequestState(ctx)
	if state.Pending() {
		return nil, sdkerrors.Wrapf(types.ErrPendingRequest, "tried to send a packet to update osmosis params but another request is pending with sequence %d", state.PacketSequence)
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
			sdk.NewAttribute(types.AttributePacketSequence, fmt.Sprintf("%d", seq)),
		))

	return &types.MsgUpdateOsmosisChainParamsResponse{PacketSequence: seq}, nil
}

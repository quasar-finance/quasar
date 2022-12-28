package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

type msgServer struct {
	Keeper
}

// NewMsgServerImpl returns an implementation of the MsgServer interface
// for the provided Keeper.
func NewMsgServerImpl(keeper Keeper) types.MsgServer {
	return &msgServer{Keeper: keeper}
}

var _ types.MsgServer = msgServer{}

func (k msgServer) UpdateChainParams(goCtx context.Context, msg *types.MsgUpdateChainParams) (*types.MsgUpdateChainParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Do not start a new procedure if module is disabled
	if !k.IsEnabled(ctx) {
		return nil, types.ErrDisabled
	}

	// TODO: Check authority of the sender

	state := k.GetRequestState(ctx, types.KeyParamsRequestState)
	if state.Pending() {
		k.Logger(ctx).Info("ignoring current osmosis chain params pending request")
	}

	seq, err := k.sendParamsRequest(ctx)
	if err != nil {
		return nil, err
	}

	return &types.MsgUpdateChainParamsResponse{PacketSequence: seq}, nil
}

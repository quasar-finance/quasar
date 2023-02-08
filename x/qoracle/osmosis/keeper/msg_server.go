package keeper

import (
	"context"

	// 	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	//	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
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
	/*
		// TODO - To be added back. Right now commented for testing with alice account.
		// Otherwise this tx should be governed by only the gov based tx.
		// Relevant tx CLI's are also added to fulfill the simple testing.
		if k.authority != msg.Creator {
			return nil, errors.Wrapf(govtypes.ErrInvalidSigner, "expected %s got %s", k.authority, msg.Creator)
		}
	*/

	ctx := sdk.UnwrapSDKContext(goCtx)
	// Do not start a new procedure if module is disabled
	if !k.IsEnabled(ctx) {
		return nil, types.ErrDisabled
	}

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

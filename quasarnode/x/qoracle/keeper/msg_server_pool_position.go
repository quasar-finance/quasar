package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (k msgServer) CreatePoolPosition(goCtx context.Context, msg *types.MsgCreatePoolPosition) (*types.MsgCreatePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value already exists
	_, isFound := k.GetPoolPosition(ctx, msg.PoolID)
	if isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "already set")
	}

	var poolPosition = types.PoolPosition{
		Creator:         msg.Creator,
		APY:             msg.APY,
		TVL:             msg.TVL,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolPosition(
		ctx,
		msg.PoolID,
		poolPosition,
	)
	return &types.MsgCreatePoolPositionResponse{}, nil
}

func (k msgServer) UpdatePoolPosition(goCtx context.Context, msg *types.MsgUpdatePoolPosition) (*types.MsgUpdatePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value exists
	valFound, isFound := k.GetPoolPosition(ctx, msg.PoolID)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "not set")
	}

	// Checks if the the msg creator is the same as the current owner
	// TODO - Oracle address check instead.
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	var poolPosition = types.PoolPosition{
		Creator:         msg.Creator,
		APY:             msg.APY,
		TVL:             msg.TVL,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolPosition(ctx, msg.PoolID, poolPosition)

	return &types.MsgUpdatePoolPositionResponse{}, nil
}

func (k msgServer) DeletePoolPosition(goCtx context.Context, msg *types.MsgDeletePoolPosition) (*types.MsgDeletePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value exists
	valFound, isFound := k.GetPoolPosition(ctx, msg.PoolID)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "not set")
	}

	// Checks if the the msg creator is the same as the current owner
	// TODO - Oracle address check instead.
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	k.RemovePoolPosition(ctx, msg.PoolID)

	return &types.MsgDeletePoolPositionResponse{}, nil
}

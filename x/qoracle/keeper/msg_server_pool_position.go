package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (k msgServer) CreatePoolPosition(goCtx context.Context, msg *types.MsgCreatePoolPosition) (*types.MsgCreatePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value already exists
	_, isFound := k.GetPoolPosition(
		ctx,
		msg.PoolId,
	)
	if isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "index already set")
	}

	var poolPosition = types.PoolPosition{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		Metrics:         msg.Metrics,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolPosition(
		ctx,
		poolPosition,
	)
	return &types.MsgCreatePoolPositionResponse{}, nil
}

func (k msgServer) UpdatePoolPosition(goCtx context.Context, msg *types.MsgUpdatePoolPosition) (*types.MsgUpdatePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}

	// Check if the value exists
	valFound, isFound := k.GetPoolPosition(
		ctx,
		msg.PoolId,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	// TODO - Oracle address check instead.
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	var poolPosition = types.PoolPosition{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		Metrics:         msg.Metrics,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolPosition(ctx, poolPosition)

	return &types.MsgUpdatePoolPositionResponse{}, nil
}

func (k msgServer) DeletePoolPosition(goCtx context.Context, msg *types.MsgDeletePoolPosition) (*types.MsgDeletePoolPositionResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}

	// Check if the value exists
	valFound, isFound := k.GetPoolPosition(
		ctx,
		msg.PoolId,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	// TODO - Oracle address check instead.
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	k.RemovePoolPosition(
		ctx,
		msg.PoolId,
	)

	return &types.MsgDeletePoolPositionResponse{}, nil
}

package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (k msgServer) CreatePoolInfo(goCtx context.Context, msg *types.MsgCreatePoolInfo) (*types.MsgCreatePoolInfoResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value already exists
	_, isFound := k.GetPoolInfo(
		ctx,
		msg.PoolId,
	)
	if isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "index already set")
	}

	var poolInfo = types.PoolInfo{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		Info:            msg.Info,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolInfo(
		ctx,
		poolInfo,
	)
	return &types.MsgCreatePoolInfoResponse{}, nil
}

func (k msgServer) UpdatePoolInfo(goCtx context.Context, msg *types.MsgUpdatePoolInfo) (*types.MsgUpdatePoolInfoResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value exists
	valFound, isFound := k.GetPoolInfo(
		ctx,
		msg.PoolId,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	var poolInfo = types.PoolInfo{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		Info:            msg.Info,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolInfo(ctx, poolInfo)

	return &types.MsgUpdatePoolInfoResponse{}, nil
}

func (k msgServer) DeletePoolInfo(goCtx context.Context, msg *types.MsgDeletePoolInfo) (*types.MsgDeletePoolInfoResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value exists
	valFound, isFound := k.GetPoolInfo(
		ctx,
		msg.PoolId,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	k.RemovePoolInfo(
		ctx,
		msg.PoolId,
	)

	return &types.MsgDeletePoolInfoResponse{}, nil
}

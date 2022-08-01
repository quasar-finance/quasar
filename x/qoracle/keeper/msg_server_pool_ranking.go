package keeper

import (
	"context"

	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (k msgServer) CreatePoolRanking(goCtx context.Context, msg *types.MsgCreatePoolRanking) (*types.MsgCreatePoolRankingResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}

	// Check if the value already exists
	_, isFound := k.GetPoolRanking(ctx)
	if isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "already set")
	}

	var poolRanking = types.PoolRanking{
		Creator:            msg.Creator,
		PoolIdsSortedByAPY: msg.PoolIdsSortedByAPY,
		PoolIdsSortedByTVL: msg.PoolIdsSortedByTVL,
		LastUpdatedTime:    msg.LastUpdatedTime,
	}

	k.SetPoolRanking(
		ctx,
		poolRanking,
	)
	return &types.MsgCreatePoolRankingResponse{}, nil
}

func (k msgServer) UpdatePoolRanking(goCtx context.Context, msg *types.MsgUpdatePoolRanking) (*types.MsgUpdatePoolRankingResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value exists
	valFound, isFound := k.GetPoolRanking(ctx)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	var poolRanking = types.PoolRanking{
		Creator:            msg.Creator,
		PoolIdsSortedByAPY: msg.PoolIdsSortedByAPY,
		PoolIdsSortedByTVL: msg.PoolIdsSortedByTVL,
		LastUpdatedTime:    msg.LastUpdatedTime,
	}

	k.SetPoolRanking(ctx, poolRanking)

	return &types.MsgUpdatePoolRankingResponse{}, nil
}

func (k msgServer) DeletePoolRanking(goCtx context.Context, msg *types.MsgDeletePoolRanking) (*types.MsgDeletePoolRankingResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	if msg.Creator != k.OracleAccounts(ctx) {
		return nil, types.ErrUnAuthorizedOracleClient
	}
	// Check if the value exists
	valFound, isFound := k.GetPoolRanking(ctx)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	k.RemovePoolRanking(ctx)

	return &types.MsgDeletePoolRankingResponse{}, nil
}

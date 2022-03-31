package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (k msgServer) CreatePoolSpotPrice(goCtx context.Context, msg *types.MsgCreatePoolSpotPrice) (*types.MsgCreatePoolSpotPriceResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value already exists
	_, isFound := k.GetPoolSpotPrice(
		ctx,
		msg.PoolId,
		msg.DenomIn,
		msg.DenomOut,
	)
	if isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "index already set")
	}

	var poolSpotPrice = types.PoolSpotPrice{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		DenomIn:         msg.DenomIn,
		DenomOut:        msg.DenomOut,
		Price:           msg.Price,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolSpotPrice(
		ctx,
		poolSpotPrice,
	)

	// Storing the stable price of a given input denom
	// Note - checking only the msg.DenomOut for stable USD denom; if it is then we are sure
	// that price of msg.DenomIn is the stable price

	stabledenoms := k.StableDenoms(ctx)
	for _, stableDenom := range stabledenoms {
		if msg.DenomOut == stableDenom {
			decPrice, err := sdk.NewDecFromStr(msg.Price)
			if err != nil {
				panic(err)
			}
			k.SetStablePrice(ctx, msg.DenomIn, decPrice)
		}
	}

	return &types.MsgCreatePoolSpotPriceResponse{}, nil
}

func (k msgServer) UpdatePoolSpotPrice(goCtx context.Context, msg *types.MsgUpdatePoolSpotPrice) (*types.MsgUpdatePoolSpotPriceResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value exists
	valFound, isFound := k.GetPoolSpotPrice(
		ctx,
		msg.PoolId,
		msg.DenomIn,
		msg.DenomOut,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	var poolSpotPrice = types.PoolSpotPrice{
		Creator:         msg.Creator,
		PoolId:          msg.PoolId,
		DenomIn:         msg.DenomIn,
		DenomOut:        msg.DenomOut,
		Price:           msg.Price,
		LastUpdatedTime: msg.LastUpdatedTime,
	}

	k.SetPoolSpotPrice(ctx, poolSpotPrice)

	return &types.MsgUpdatePoolSpotPriceResponse{}, nil
}

func (k msgServer) DeletePoolSpotPrice(goCtx context.Context, msg *types.MsgDeletePoolSpotPrice) (*types.MsgDeletePoolSpotPriceResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Check if the value exists
	valFound, isFound := k.GetPoolSpotPrice(
		ctx,
		msg.PoolId,
		msg.DenomIn,
		msg.DenomOut,
	)
	if !isFound {
		return nil, sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, "index not set")
	}

	// Checks if the the msg creator is the same as the current owner
	if msg.Creator != valFound.Creator {
		return nil, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "incorrect owner")
	}

	k.RemovePoolSpotPrice(
		ctx,
		msg.PoolId,
		msg.DenomIn,
		msg.DenomOut,
	)

	return &types.MsgDeletePoolSpotPriceResponse{}, nil
}

package keeper

import (
	"context"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) ICAAddressOnDenomNativeZone(goCtx context.Context, req *types.QueryICAAddressOnDenomNativeZoneRequest) (*types.QueryICAAddressOnDenomNativeZoneResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	if _, err := sdk.AccAddressFromBech32(req.Owner); err != nil {
		return nil, sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid owner address (%s)", err)
	}

	if err := sdk.ValidateDenom(req.Denom); err != nil {
		return nil, sdkerrors.Wrap(types.ErrInvalidDenom, err.Error())
	}

	address, found := k.IsICACreatedOnDenomNativeZone(ctx, req.Denom, req.Owner)
	if !found {
		return nil, sdkerrors.Wrapf(types.ErrICANotFound, "no ICA owned by %s is found on native zone od denom %s", req.Owner, req.Denom)
	}

	return &types.QueryICAAddressOnDenomNativeZoneResponse{IcaAddress: address}, nil
}

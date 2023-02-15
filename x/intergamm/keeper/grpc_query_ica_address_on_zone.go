package keeper

import (
	"context"

	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) ICAAddressOnZone(goCtx context.Context, req *types.QueryICAAddressOnZoneRequest) (*types.QueryICAAddressOnZoneResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	if _, err := sdk.AccAddressFromBech32(req.Owner); err != nil {
		return nil, errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid owner address (%s)", err)
	}

	address, found := k.IsICACreatedOnZoneId(ctx, req.ZoneId, req.Owner)
	if !found {
		return nil, errors.Wrapf(types.ErrICANotFound, "no ICA owned by %s is found on zone %s", req.Owner, req.ZoneId)
	}

	return &types.QueryICAAddressOnZoneResponse{IcaAddress: address}, nil
}

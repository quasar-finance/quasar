package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v6/modules/apps/27-interchain-accounts/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (ms msgServer) RegisterInterchainAccount(goCtx context.Context, register *types.MsgRegisterInterchainAccount) (*types.MsgRegisterInterchainAccountResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	creator, err := sdk.AccAddressFromBech32(register.Creator)
	if err != nil {
		return nil, err
	}

	portID, err := icatypes.NewControllerPortID(creator.String())
	if err != nil {
		return nil, status.Errorf(codes.InvalidArgument, "could not generate port for address: %s", err)
	}

	_, found := ms.k.icaControllerKeeper.GetOpenActiveChannel(ctx, register.ConnectionId, portID)
	if !found {
		err = ms.k.RegisterInterchainAccount(ctx, register.ConnectionId, creator.String())
		if err != nil {
			return nil, err
		}
	}
	return &types.MsgRegisterInterchainAccountResponse{}, nil
}

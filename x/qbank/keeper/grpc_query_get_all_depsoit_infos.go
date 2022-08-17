package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) GetAllDepsoitInfos(goCtx context.Context, req *types.QueryGetAllDepsoitInfosRequest) (*types.QueryGetAllDepsoitInfosResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)
	var res []types.DepositInfo
	if req.VaultID == oriontypes.ModuleName {
		res = k.GetAllDepositInfos(ctx)
	}

	return &types.QueryGetAllDepsoitInfosResponse{DepositInfos: res}, nil
}

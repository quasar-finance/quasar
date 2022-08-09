package keeper

import (
	"context"

	balancerpool "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (k Keeper) OsmosisPools(goCtx context.Context, req *types.QueryOsmosisPoolsRequest) (*types.QueryOsmosisPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	var pools []balancerpool.Pool
	store := prefix.NewStore(k.getOsmosisStore(ctx), types.KeyOsmosisPoolPrefix)
	pageRes, err := query.Paginate(store, req.Pagination, func(key []byte, value []byte) error {
		var pool balancerpool.Pool
		if err := k.cdc.Unmarshal(value, &pool); err != nil {
			return err
		}

		pools = append(pools, pool)
		return nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryOsmosisPoolsResponse{
		Pools:      pools,
		Pagination: pageRes,
	}, nil
}

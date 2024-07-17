package keeper

import (
	"context"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"cosmossdk.io/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"

	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

var _ types.QueryServer = Keeper{}

// Params implements the Query/Params gRPC method
func (q Keeper) Params(c context.Context, _ *types.QueryParamsRequest) (*types.QueryParamsResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)
	params := q.GetParams(ctx)

	return &types.QueryParamsResponse{
		Params: params,
	}, nil
}

func (q Keeper) State(c context.Context, _ *types.QueryStateRequest) (*types.QueryStateResponse, error) {
	ctx := sdk.UnwrapSDKContext(c)

	return &types.QueryStateResponse{
		ParamsRequestState:     q.GetRequestState(ctx, types.KeyParamsRequestState),
		IncentivizedPoolsState: q.GetRequestState(ctx, types.KeyIncentivizedPoolsRequestState),
		PoolsState:             q.GetRequestState(ctx, types.KeyPoolsRequestState),
	}, nil
}

func (q Keeper) ChainParams(goCtx context.Context, req *types.QueryChainParamsRequest) (*types.QueryChainParamsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}
	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryChainParamsResponse{
		EpochsInfo:          q.GetEpochsInfo(ctx),
		LockableDurations:   q.GetLockableDurations(ctx),
		MintParams:          q.GetMintParams(ctx),
		MintEpochProvisions: q.GetMintEpochProvisions(ctx),
		DistrInfo:           q.GetDistrInfo(ctx),
	}, nil
}

func (q Keeper) IncentivizedPools(goCtx context.Context, req *types.QueryIncentivizedPoolsRequest) (*types.QueryIncentivizedPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	return &types.QueryIncentivizedPoolsResponse{
		IncentivizedPools: q.GetIncentivizedPools(ctx),
	}, nil
}

func (q Keeper) Pools(goCtx context.Context, req *types.QueryPoolsRequest) (*types.QueryPoolsResponse, error) {
	if req == nil {
		return nil, status.Error(codes.InvalidArgument, "invalid request")
	}

	ctx := sdk.UnwrapSDKContext(goCtx)

	var pools []balancerpool.Pool
	store := prefix.NewStore(ctx.KVStore(q.storeKey), types.KeyPoolPrefix)
	pageRes, err := query.Paginate(store, req.Pagination, func(key []byte, value []byte) error {
		var pool balancerpool.Pool
		if err := q.cdc.Unmarshal(value, &pool); err != nil {
			return err
		}

		pools = append(pools, pool)
		return nil
	})
	if err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}

	return &types.QueryPoolsResponse{
		Pools:      pools,
		Pagination: pageRes,
	}, nil
}

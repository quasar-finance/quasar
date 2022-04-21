package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle/types"
)

func TestPoolInfoQuerySingle(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQoracleKeeper()
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolInfo(&keeper, ctx, 2)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetPoolInfoRequest
		response *types.QueryGetPoolInfoResponse
		err      error
	}{
		{
			desc: "First",
			request: &types.QueryGetPoolInfoRequest{
				PoolId: msgs[0].PoolId,
			},
			response: &types.QueryGetPoolInfoResponse{PoolInfo: msgs[0]},
		},
		{
			desc: "Second",
			request: &types.QueryGetPoolInfoRequest{
				PoolId: msgs[1].PoolId,
			},
			response: &types.QueryGetPoolInfoResponse{PoolInfo: msgs[1]},
		},
		{
			desc: "KeyNotFound",
			request: &types.QueryGetPoolInfoRequest{
				PoolId: "100000",
			},
			err: status.Error(codes.InvalidArgument, "not found"),
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.PoolInfo(wctx, tc.request)
			if tc.err != nil {
				require.ErrorIs(t, err, tc.err)
			} else {
				require.NoError(t, err)
				require.Equal(t,
					nullify.Fill(tc.response),
					nullify.Fill(response),
				)
			}
		})
	}
}

func TestPoolInfoQueryPaginated(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQoracleKeeper()
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolInfo(&keeper, ctx, 5)

	request := func(next []byte, offset, limit uint64, total bool) *types.QueryAllPoolInfoRequest {
		return &types.QueryAllPoolInfoRequest{
			Pagination: &query.PageRequest{
				Key:        next,
				Offset:     offset,
				Limit:      limit,
				CountTotal: total,
			},
		}
	}
	t.Run("ByOffset", func(t *testing.T) {
		step := 2
		for i := 0; i < len(msgs); i += step {
			resp, err := keeper.PoolInfoAll(wctx, request(nil, uint64(i), uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolInfo), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolInfo),
			)
		}
	})
	t.Run("ByKey", func(t *testing.T) {
		step := 2
		var next []byte
		for i := 0; i < len(msgs); i += step {
			resp, err := keeper.PoolInfoAll(wctx, request(next, 0, uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolInfo), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolInfo),
			)
			next = resp.Pagination.NextKey
		}
	})
	t.Run("Total", func(t *testing.T) {
		resp, err := keeper.PoolInfoAll(wctx, request(nil, 0, 0, true))
		require.NoError(t, err)
		require.Equal(t, len(msgs), int(resp.Pagination.Total))
		require.ElementsMatch(t,
			nullify.Fill(msgs),
			nullify.Fill(resp.PoolInfo),
		)
	})
	t.Run("InvalidRequest", func(t *testing.T) {
		_, err := keeper.PoolInfoAll(wctx, nil)
		require.ErrorIs(t, err, status.Error(codes.InvalidArgument, "invalid request"))
	})
}

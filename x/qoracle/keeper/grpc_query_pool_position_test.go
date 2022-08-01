package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func TestPoolPositionQuerySingle(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolPosition(&k, ctx, 2)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetPoolPositionRequest
		response *types.QueryGetPoolPositionResponse
		err      error
	}{
		{
			desc: "First",
			request: &types.QueryGetPoolPositionRequest{
				PoolId: msgs[0].PoolId,
			},
			response: &types.QueryGetPoolPositionResponse{PoolPosition: msgs[0]},
		},
		{
			desc: "Second",
			request: &types.QueryGetPoolPositionRequest{
				PoolId: msgs[1].PoolId,
			},
			response: &types.QueryGetPoolPositionResponse{PoolPosition: msgs[1]},
		},
		{
			desc: "KeyNotFound",
			request: &types.QueryGetPoolPositionRequest{
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
			response, err := k.PoolPosition(wctx, tc.request)
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

func TestPoolPositionQueryPaginated(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolPosition(&k, ctx, 5)

	request := func(next []byte, offset, limit uint64, total bool) *types.QueryAllPoolPositionRequest {
		return &types.QueryAllPoolPositionRequest{
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
			resp, err := k.PoolPositionAll(wctx, request(nil, uint64(i), uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolPosition), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolPosition),
			)
		}
	})
	t.Run("ByKey", func(t *testing.T) {
		step := 2
		var next []byte
		for i := 0; i < len(msgs); i += step {
			resp, err := k.PoolPositionAll(wctx, request(next, 0, uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolPosition), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolPosition),
			)
			next = resp.Pagination.NextKey
		}
	})
	t.Run("Total", func(t *testing.T) {
		resp, err := k.PoolPositionAll(wctx, request(nil, 0, 0, true))
		require.NoError(t, err)
		require.Equal(t, len(msgs), int(resp.Pagination.Total))
		require.ElementsMatch(t,
			nullify.Fill(msgs),
			nullify.Fill(resp.PoolPosition),
		)
	})
	t.Run("InvalidRequest", func(t *testing.T) {
		_, err := k.PoolPositionAll(wctx, nil)
		require.ErrorIs(t, err, status.Error(codes.InvalidArgument, "invalid request"))
	})
}

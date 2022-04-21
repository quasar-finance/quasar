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

func TestPoolSpotPriceQuerySingle(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQoracleKeeper()
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolSpotPrice(&keeper, ctx, 2)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetPoolSpotPriceRequest
		response *types.QueryGetPoolSpotPriceResponse
		err      error
	}{
		{
			desc: "First",
			request: &types.QueryGetPoolSpotPriceRequest{
				PoolId:   msgs[0].PoolId,
				DenomIn:  msgs[0].DenomIn,
				DenomOut: msgs[0].DenomOut,
			},
			response: &types.QueryGetPoolSpotPriceResponse{PoolSpotPrice: msgs[0]},
		},
		{
			desc: "Second",
			request: &types.QueryGetPoolSpotPriceRequest{
				PoolId:   msgs[1].PoolId,
				DenomIn:  msgs[1].DenomIn,
				DenomOut: msgs[1].DenomOut,
			},
			response: &types.QueryGetPoolSpotPriceResponse{PoolSpotPrice: msgs[1]},
		},
		{
			desc: "KeyNotFound",
			request: &types.QueryGetPoolSpotPriceRequest{
				PoolId:   "100000",
				DenomIn:  "xyz",
				DenomOut: "zyx",
			},
			err: status.Error(codes.InvalidArgument, "not found"),
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.PoolSpotPrice(wctx, tc.request)
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

func TestPoolSpotPriceQueryPaginated(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetQoracleKeeper()
	wctx := sdk.WrapSDKContext(ctx)
	msgs := createNPoolSpotPrice(&keeper, ctx, 5)

	request := func(next []byte, offset, limit uint64, total bool) *types.QueryAllPoolSpotPriceRequest {
		return &types.QueryAllPoolSpotPriceRequest{
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
			resp, err := keeper.PoolSpotPriceAll(wctx, request(nil, uint64(i), uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolSpotPrice), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolSpotPrice),
			)
		}
	})
	t.Run("ByKey", func(t *testing.T) {
		step := 2
		var next []byte
		for i := 0; i < len(msgs); i += step {
			resp, err := keeper.PoolSpotPriceAll(wctx, request(next, 0, uint64(step), false))
			require.NoError(t, err)
			require.LessOrEqual(t, len(resp.PoolSpotPrice), step)
			require.Subset(t,
				nullify.Fill(msgs),
				nullify.Fill(resp.PoolSpotPrice),
			)
			next = resp.Pagination.NextKey
		}
	})
	t.Run("Total", func(t *testing.T) {
		resp, err := keeper.PoolSpotPriceAll(wctx, request(nil, 0, 0, true))
		require.NoError(t, err)
		require.Equal(t, len(msgs), int(resp.Pagination.Total))
		require.ElementsMatch(t,
			nullify.Fill(msgs),
			nullify.Fill(resp.PoolSpotPrice),
		)
	})
	t.Run("InvalidRequest", func(t *testing.T) {
		_, err := keeper.PoolSpotPriceAll(wctx, nil)
		require.ErrorIs(t, err, status.Error(codes.InvalidArgument, "invalid request"))
	})
}

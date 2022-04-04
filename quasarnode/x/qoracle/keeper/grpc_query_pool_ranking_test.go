package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qoracle/types"
)

func TestPoolRankingQuery(t *testing.T) {
	keeper, ctx := keepertest.QoracleKeeper(t)
	wctx := sdk.WrapSDKContext(ctx)
	item := createTestPoolRanking(keeper, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetPoolRankingRequest
		response *types.QueryGetPoolRankingResponse
		err      error
	}{
		{
			desc:     "First",
			request:  &types.QueryGetPoolRankingRequest{},
			response: &types.QueryGetPoolRankingResponse{PoolRanking: item},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.PoolRanking(wctx, tc.request)
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

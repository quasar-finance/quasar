package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/osmolpv/types"
)

func TestLpPositionQuery(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	wctx := sdk.WrapSDKContext(ctx)
	item := createTestLpPosition(keeper, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetLpPositionRequest
		response *types.QueryGetLpPositionResponse
		err      error
	}{
		{
			desc:     "First",
			request:  &types.QueryGetLpPositionRequest{},
			response: &types.QueryGetLpPositionResponse{LpPosition: item},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.LpPosition(wctx, tc.request)
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

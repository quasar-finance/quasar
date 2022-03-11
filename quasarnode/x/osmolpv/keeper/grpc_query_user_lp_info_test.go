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

func TestUserLPInfoQuery(t *testing.T) {
	keeper, ctx := keepertest.OsmolpvKeeper(t)
	wctx := sdk.WrapSDKContext(ctx)
	item := createTestUserLPInfo(keeper, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetUserLPInfoRequest
		response *types.QueryGetUserLPInfoResponse
		err      error
	}{
		{
			desc:     "First",
			request:  &types.QueryGetUserLPInfoRequest{},
			response: &types.QueryGetUserLPInfoResponse{UserLPInfo: item},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.UserLPInfo(wctx, tc.request)
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

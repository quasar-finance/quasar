package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/types"
)

func TestEpochLPInfoQuery(t *testing.T) {
	ctx, keeper := keepertest.NewTestSetup(t).GetOrionKeeper()
	wctx := sdk.WrapSDKContext(ctx)
	item := createTestEpochLPInfo(&keeper, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetEpochLPInfoRequest
		response *types.QueryGetEpochLPInfoResponse
		err      error
	}{
		{
			desc: "First",
			request: &types.QueryGetEpochLPInfoRequest{
				EpochDay: item.EpochDay,
			},
			response: &types.QueryGetEpochLPInfoResponse{EpochLPInfo: item},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.EpochLPInfo(wctx, tc.request)
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

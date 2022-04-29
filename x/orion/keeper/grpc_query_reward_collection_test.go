package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/types"
)

func TestRewardCollectionQuery(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	wctx := sdk.WrapSDKContext(ctx)

	item, epochDay := createTestRewardCollection(&k, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetRewardCollectionRequest
		response *types.QueryGetRewardCollectionResponse
		err      error
	}{
		{
			desc: "First",
			request: &types.QueryGetRewardCollectionRequest{
				EpochDay: epochDay,
			},
			response: &types.QueryGetRewardCollectionResponse{
				RewardCollection: item,
			},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := k.RewardCollection(wctx, tc.request)
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

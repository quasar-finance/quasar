package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/qbank/types"
)

func TestFeeDataQuery(t *testing.T) {
	keeper, ctx := keepertest.QbankKeeper(t)
	wctx := sdk.WrapSDKContext(ctx)
	item := createTestFeeData(keeper, ctx)
	for _, tc := range []struct {
		desc     string
		request  *types.QueryGetFeeDataRequest
		response *types.QueryGetFeeDataResponse
		err      error
	}{
		{
			desc:     "First",
			request:  &types.QueryGetFeeDataRequest{},
			response: &types.QueryGetFeeDataResponse{FeeData: item},
		},
		{
			desc: "InvalidRequest",
			err:  status.Error(codes.InvalidArgument, "invalid request"),
		},
	} {
		t.Run(tc.desc, func(t *testing.T) {
			response, err := keeper.FeeData(wctx, tc.request)
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

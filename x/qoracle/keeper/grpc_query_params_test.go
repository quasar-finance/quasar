package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/stretchr/testify/require"
)

func TestParamsQuery(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	wctx := sdk.WrapSDKContext(ctx)
	params := types.DefaultParams()
	k.SetParams(ctx, params)

	response, err := k.Params(wctx, &types.QueryParamsRequest{})
	require.NoError(t, err)
	require.Equal(t, &types.QueryParamsResponse{Params: params}, response)
}

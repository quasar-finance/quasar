package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/stretchr/testify/require"
)

func TestParamsQuery(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	wctx := sdk.WrapSDKContext(ctx)
	params := types.DefaultParams()
	k.SetParams(ctx, params)

	response, err := k.Params(wctx, &types.QueryParamsRequest{})
	require.NoError(t, err)
	require.True(t, params.Equal(response.Params))
}

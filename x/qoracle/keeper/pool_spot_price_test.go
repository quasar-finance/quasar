package keeper_test

import (
	"fmt"
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func createNPoolSpotPrice(keeper *keeper.Keeper, ctx sdk.Context, n int) []types.PoolSpotPrice {
	items := make([]types.PoolSpotPrice, n)
	for i := range items {
		items[i].Creator = sample.AccAddressStr()
		items[i].PoolId = fmt.Sprintf("%d", i)
		items[i].DenomIn = fmt.Sprintf("abc%d", i)
		items[i].DenomOut = fmt.Sprintf("cba%d", i)
		items[i].Price = fmt.Sprintf("%f", 1.5*float32(i))
		items[i].LastUpdatedTime = 1 + uint64(i)

		keeper.SetPoolSpotPrice(ctx, items[i])
	}
	return items
}

type DenomPrice struct {
	Denom string
	Price sdk.Dec
}

func createStablePrice(k *keeper.Keeper, ctx sdk.Context) DenomPrice {
	price, _ := sdk.NewDecFromStr("10.12")
	dp := DenomPrice{Denom: "testd_enom_1", Price: price}

	k.SetStablePrice(ctx, dp.Denom, dp.Price)

	return dp
}
func TestPoolSpotPriceGet(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolSpotPrice(&k, ctx, 10)
	for _, item := range items {
		rst, found := k.GetPoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		require.True(t, found)
		require.Equal(t,
			nullify.Fill(&item),
			nullify.Fill(&rst),
		)
	}
}
func TestPoolSpotPriceRemove(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolSpotPrice(&k, ctx, 10)
	for _, item := range items {
		k.RemovePoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		_, found := k.GetPoolSpotPrice(ctx,
			item.PoolId,
			item.DenomIn,
			item.DenomOut,
		)
		require.False(t, found)
	}
}

func TestPoolSpotPriceGetAll(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	items := createNPoolSpotPrice(&k, ctx, 10)
	require.ElementsMatch(t,
		nullify.Fill(items),
		nullify.Fill(k.GetAllPoolSpotPrice(ctx)),
	)
}

func TestStablePrice(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	// Input
	inputDP1 := createStablePrice(&k, ctx)
	inputDPS := []DenomPrice{inputDP1}

	// Outputs
	price1, found := k.GetStablePrice(ctx, inputDP1.Denom)
	require.True(t, found)
	var outputDPS []DenomPrice
	outputDPS = append(outputDPS, DenomPrice{Denom: inputDP1.Denom, Price: price1})
	require.ElementsMatch(t,
		nullify.Fill(inputDPS),
		nullify.Fill(outputDPS),
	)
}

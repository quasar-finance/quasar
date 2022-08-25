package keeper_test

import (
	"testing"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/testutil/nullify"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

type DenomPrice struct {
	Denom string
	Price sdk.Dec
}

func createStablePrice(k *keeper.Keeper, ctx sdk.Context) DenomPrice {
	price, _ := sdk.NewDecFromStr("10.12")
	dp := DenomPrice{Denom: "testdenom1", Price: price}

	k.SetStablePrice(ctx, dp.Denom, dp.Price)

	return dp
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

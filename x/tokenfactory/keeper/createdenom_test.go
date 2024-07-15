package keeper_test

import (
	"fmt"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func TestMsgCreateDenom(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	fundAccsAmount := sdk.NewCoins(sdk.NewCoin(types.DefaultParams().DenomCreationFee[0].Denom,
		types.DefaultParams().DenomCreationFee[0].Amount.MulRaw(100)), sdk.NewCoin("uosmo", sdk.NewInt(10000000)))
	for _, acc := range setup.TestAccs {
		setup.FundAcc(t, acc, fundAccsAmount)
	}
	ctx, tokenFactoryKeeper := setup.Ctx, setup.Keepers.TfKeeper
	bankKeeper := setup.Keepers.BankKeeper
	params := types.DefaultParams()

	tokenFactoryKeeper.SetParams(ctx, params)
	denomCreationFee := tokenFactoryKeeper.GetParams(ctx).DenomCreationFee

	// Get balance of acc 0 before creating a denom
	// preCreateBalance := bankKeeper.GetBalance(ctx, suite.TestAccs[0], denomCreationFee[0].Denom)
	preCreateBalance := bankKeeper.GetBalance(ctx, setup.TestAccs[0], denomCreationFee[0].Denom)
	fmt.Printf("balance=%v", preCreateBalance)
	res, err := tokenFactoryKeeper.CreateDenom(ctx, setup.TestAccs[0].String(), "bitcoin")
	require.NoError(t, err)
	require.NotEmpty(t, res)

	// Make sure that the admin is set correctly
	denom_metadata, err := tokenFactoryKeeper.GetAuthorityMetadata(ctx, res)
	require.NoError(t, err)
	require.NotEmpty(t, denom_metadata)
	require.Equal(t, setup.TestAccs[0].String(), denom_metadata.Admin)

	postCreateBalance := bankKeeper.GetBalance(ctx, setup.TestAccs[0], tokenFactoryKeeper.GetParams(ctx).DenomCreationFee[0].Denom)
	require.True(t, preCreateBalance.Sub(postCreateBalance).IsEqual(denomCreationFee[0]))

	// Make sure that a second version of the same denom can't be recreated
	res1, err1 := tokenFactoryKeeper.CreateDenom(ctx, setup.TestAccs[0].String(), "bitcoin")
	require.Error(t, err1)
	require.Empty(t, res1)
}

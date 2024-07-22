package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func TestGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, tokenFactoryKeeper := setup.Ctx, setup.Keepers.TfKeeper
	bankKeeper := setup.Keepers.BankKeeper
	accountKeeper := setup.Keepers.AccountKeeper
	genesisState := types.GenesisState{
		FactoryDenoms: []types.GenesisDenom{
			{
				Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/bitcoin",
				AuthorityMetadata: types.DenomAuthorityMetadata{
					Admin: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
				},
			},
			{
				Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/diff-admin",
				AuthorityMetadata: types.DenomAuthorityMetadata{
					Admin: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
				},
			},
			{
				Denom: "factory/quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec/litecoin",
				AuthorityMetadata: types.DenomAuthorityMetadata{
					Admin: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec",
				},
			},
		},
	}

	// Test both with bank denom metadata set, and not set.
	for i, denom := range genesisState.FactoryDenoms {
		// hacky, sets bank metadata to exist if i != 0, to cover both cases.
		if i != 0 {
			bankKeeper.SetDenomMetaData(ctx, banktypes.Metadata{Base: denom.GetDenom()})
		}
	}

	// check before initGenesis that the module account is nil
	tokenfactoryModuleAccount := accountKeeper.GetAccount(ctx, accountKeeper.GetModuleAddress(types.ModuleName))
	require.Nil(t, tokenfactoryModuleAccount)

	tokenFactoryKeeper.SetParams(ctx, types.Params{DenomCreationFee: sdk.Coins{sdk.NewInt64Coin("uosmo", 100)}})
	tokenFactoryKeeper.InitGenesis(ctx, genesisState)

	// check that the module account is now initialized
	tokenfactoryModuleAccount = accountKeeper.GetAccount(ctx, accountKeeper.GetModuleAddress(types.ModuleName))
	require.NotNil(t, tokenfactoryModuleAccount)

	exportedGenesis := tokenFactoryKeeper.ExportGenesis(ctx)
	require.NotNil(t, exportedGenesis)
	require.Equal(t, genesisState, *exportedGenesis)
}

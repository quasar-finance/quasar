package osmolpv

import (
	"github.com/abag/quasarnode/x/osmolpv/keeper"
	"github.com/abag/quasarnode/x/osmolpv/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
// state.
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {
	// Set if defined
	if genState.FeeData != nil {
		k.SetFeeData(ctx, *genState.FeeData)
	}
	// this line is used by starport scaffolding # genesis/module/init
	k.SetParams(ctx, genState.Params)

	// Prepare list of strategies.
	// As of now they are not added in the genesis State variable but can be added if
	// required.
	// The Procedure of adding new names in future should be done via a software upgrade
	// procedure. New names should be added here whenever a new strategy is launched.
	strategyNames := []string{types.MeissaStrategyName, types.RigelStrategyName}
	k.SetStrategyNames(ctx, strategyNames)
}

// ExportGenesis returns the capability module's exported genesis.
func ExportGenesis(ctx sdk.Context, k keeper.Keeper) *types.GenesisState {
	genesis := types.DefaultGenesis()
	genesis.Params = k.GetParams(ctx)

	// Get all feeData
	feeData, found := k.GetFeeData(ctx)
	if found {
		genesis.FeeData = &feeData
	}
	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

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

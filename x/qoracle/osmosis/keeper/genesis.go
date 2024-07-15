package keeper

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"

	genesistypes "github.com/quasarlabs/quasarnode/x/qoracle/genesis/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
// state.
func InitGenesis(ctx sdk.Context, k Keeper, genState genesistypes.OsmosisGenesisState) {
	k.SetPort(ctx, genState.Port)

	// Only try to bind to port if it is not already bound, since we may already own
	// port capability from capability InitGenesis
	if !k.IsBound(ctx, genState.Port) {
		err := k.BindPort(ctx, genState.Port)
		if err != nil {
			panic(fmt.Sprintf("could not claim port capability: %v", err))
		}
	}

	k.SetParams(ctx, genState.Params)
}

// ExportGenesis returns the capability module's exported genesis.
func ExportGenesis(ctx sdk.Context, k Keeper) genesistypes.OsmosisGenesisState {
	return genesistypes.NewOsmosisGenesisState(
		k.GetPort(ctx),
		k.GetParams(ctx),
	)
}

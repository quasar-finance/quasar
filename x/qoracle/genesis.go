package qoracle

import (
	"fmt"

	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
// state.
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {
	k.SetPort(ctx, genState.PortId)

	// Only try to bind to port if it is not already bound, since we may already own
	// port capability from capability InitGenesis
	if !k.IsBound(ctx, genState.PortId) {
		// transfer module binds to the transfer port on InitChain
		// and claims the returned capability
		err := k.BindPort(ctx, genState.PortId)
		if err != nil {
			panic(fmt.Sprintf("could not claim port capability: %v", err))
		}
	}

	// Set all the poolPosition
	for _, elem := range genState.PoolPositionList {
		k.SetPoolPosition(ctx, elem)
	}
	// Set if defined
	if genState.PoolRanking != nil {
		k.SetPoolRanking(ctx, *genState.PoolRanking)
	}
	// Set all the poolSpotPrice
	for _, elem := range genState.PoolSpotPriceList {
		k.SetPoolSpotPrice(ctx, elem)
	}
	// Set all the poolInfo
	for _, elem := range genState.PoolInfoList {
		k.SetPoolInfo(ctx, elem)
	}
	// this line is used by starport scaffolding # genesis/module/init
	k.SetParams(ctx, genState.Params)
}

// ExportGenesis returns the capability module's exported genesis.
func ExportGenesis(ctx sdk.Context, k keeper.Keeper) *types.GenesisState {
	genesis := types.DefaultGenesis()
	genesis.PortId = k.GetPort(ctx)
	genesis.Params = k.GetParams(ctx)

	genesis.PoolPositionList = k.GetAllPoolPosition(ctx)
	// Get all poolRanking
	poolRanking, found := k.GetPoolRanking(ctx)
	if found {
		genesis.PoolRanking = &poolRanking
	}
	genesis.PoolSpotPriceList = k.GetAllPoolSpotPrice(ctx)
	genesis.PoolInfoList = k.GetAllPoolInfo(ctx)
	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

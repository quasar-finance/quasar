package qbank

import (
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
// state.
// should be used. Genesis state should have all other KV stores informations.
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {

	// this line is used by starport scaffolding # genesis/module/init
	k.SetParams(ctx, genState.Params)
}

// ExportGenesis returns the capability module's exported genesis.
// TODO | AUDIT | Export genesis should export all other KV store info in an appropriate struct object.
func ExportGenesis(ctx sdk.Context, k keeper.Keeper) *types.GenesisState {
	genesis := types.DefaultGenesis()
	genesis.Params = k.GetParams(ctx)

	// Prepare the deposit info state
	genesis.DepositInfos = k.GetAllDepositInfos(ctx)
	genesis.TotalDeposits = k.GetAllTotalDeposits(ctx)
	genesis.Withdrawables = k.GetAllActualWithdrawables(ctx)
	genesis.TotalWithdraws = k.GetAllTotalWithdraws(ctx)
	genesis.ClaimableRewards = k.GetAllClaimableRewards(ctx)
	genesis.TotalClaimedRewards = k.GetAllTotalClaimedRewards(ctx)

	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

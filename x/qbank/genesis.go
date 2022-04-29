package qbank

import (
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {
	// this line is used by starport scaffolding # genesis/module/init
	k.SetParams(ctx, genState.Params)

	for _, di := range genState.DepositInfos {
		k.AddEpochLockupUserDenomDeposit(ctx,
			di.DepositorAccAddress,
			di.Coin,
			di.EpochDay,
			di.LockupPeriod)
	}

	for _, td := range genState.TotalDeposits {
		for _, coin := range td.Coins {
			k.AddUserDeposit(ctx, td.DepositorAccAddress, coin)
			k.AddUserDenomDeposit(ctx, td.DepositorAccAddress, coin)
		}
	}

	for _, w := range genState.Withdrawables {
		for _, coin := range w.Coins {
			k.AddActualWithdrawableAmt(ctx, w.DepositorAccAddress, coin)
		}
	}

	for _, tw := range genState.TotalWithdraws {
		k.AddTotalWithdrawAmt(ctx, tw.DepositorAccAddress, tw.VaultID, tw.Coins)
	}

	for _, cr := range genState.ClaimableRewards {
		k.AddUserClaimRewards(ctx, cr.DepositorAccAddress, cr.VaultID, cr.Coins)
	}

	for _, tcr := range genState.TotalClaimedRewards {
		k.AddUserClaimedRewards(ctx, tcr.DepositorAccAddress, tcr.VaultID, tcr.Coins)
	}
}

// ExportGenesis returns the capability module's exported genesis.
func ExportGenesis(ctx sdk.Context, k keeper.Keeper) *types.GenesisState {
	genesis := types.DefaultGenesis()

	genesis.Params = k.GetParams(ctx)
	genesis.DepositInfos = k.GetAllDepositInfos(ctx)
	genesis.TotalDeposits = k.GetAllTotalDeposits(ctx)
	genesis.Withdrawables = k.GetAllActualWithdrawables(ctx)
	genesis.TotalWithdraws = k.GetAllTotalWithdraws(ctx)
	genesis.ClaimableRewards = k.GetAllClaimableRewards(ctx)
	genesis.TotalClaimedRewards = k.GetAllTotalClaimedRewards(ctx)

	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

package qbank

import (
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// InitGenesis initializes the capability module's state from a provided genesis
// state.
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {
	// Set all the deposit
	for _, elem := range genState.DepositList {
		k.SetDeposit(ctx, elem)
	}

	// Set deposit count
	k.SetDepositCount(ctx, genState.DepositCount)
	// Set all the withdraw
	for _, elem := range genState.WithdrawList {
		k.SetWithdraw(ctx, elem)
	}

	// Set withdraw count
	k.SetWithdrawCount(ctx, genState.WithdrawCount)
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

	genesis.DepositList = k.GetAllDeposit(ctx)
	genesis.DepositCount = k.GetDepositCount(ctx)
	genesis.WithdrawList = k.GetAllWithdraw(ctx)
	genesis.WithdrawCount = k.GetWithdrawCount(ctx)
	// Get all feeData
	feeData, found := k.GetFeeData(ctx)
	if found {
		genesis.FeeData = &feeData
	}
	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

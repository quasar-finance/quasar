package osmolpv

import (
	"github.com/abag/quasarnode/x/osmolpv/keeper"
	"github.com/abag/quasarnode/x/osmolpv/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// TODO | AUDIT | Genesis state is not yet defined.

// InitGenesis initializes the capability module's state from a provided genesis
// state.
func InitGenesis(ctx sdk.Context, k keeper.Keeper, genState types.GenesisState) {
	// Set if defined
	if genState.FeeData != nil {
		// k.SetFeeData(ctx, *genState.FeeData)
	}
	// Set if defined
	/*
		if genState.LpPosition != nil {
			k.setLpPosition(ctx, *genState.LpPosition)
		}
	*/
	// Set if defined
	if genState.EpochLPInfo != nil {
		k.SetEpochLPInfo(ctx, *genState.EpochLPInfo)
	}
	// Set if defined
	// TODO | AUDIT | Genesis state is to be defined
	/*
		if genState.RewardCollection != nil {
			k.SetRewardCollection(ctx, *genState.RewardCollection)
		}
	*/
	// Set if defined
	/*
		if genState.UserLPInfo != nil {
			k.SetUserLPInfo(ctx, *genState.UserLPInfo)
		}
	*/
	// Set if defined
	/*
		if genState.LpStat != nil {
			k.SetLpStat(ctx, *genState.LpStat)
		}
	*/
	// this line is used by starport scaffolding # genesis/module/init
	k.SetParams(ctx, genState.Params)

	// Prepare list of strategies.
	// As of now they are not added in the genesis State variable but can be added if
	// required.
	// The Procedure of adding new names in future should be done via a software upgrade
	// procedure. New names should be added here whenever a new strategy is launched.
	strategyNames := []string{types.MeissaStrategyName, types.RigelStrategyName}
	k.SetStrategyNames(ctx, strategyNames)
	meissaSubNames := []string{types.Meissa7d, types.Meissa21d, types.Meissa1m, types.Meissa3m}
	k.SetSubStrategyNames(ctx, types.MeissaStrategyName, meissaSubNames)
}

// ExportGenesis returns the capability module's exported genesis.
func ExportGenesis(ctx sdk.Context, k keeper.Keeper) *types.GenesisState {
	genesis := types.DefaultGenesis()
	genesis.Params = k.GetParams(ctx)

	// Get all feeData
	/*
		feeData, found := k.GetFeeData(ctx)
		if found {
			genesis.FeeData = &feeData
		}
	*/
	// TODO | AUDIT | Genesis state to be defined
	/*
		// Get all lpPosition
		lpPosition, found := k.GetLpPosition(ctx)
		if found {
			genesis.LpPosition = &lpPosition
		}
		// Get all epochLPInfo
		epochLPInfo, found := k.GetEpochLPInfo(ctx)
		if found {
			genesis.EpochLPInfo = &epochLPInfo
		}
		// Get all rewardCollection
		rewardCollection, found := k.GetRewardCollection(ctx)
		if found {
			genesis.RewardCollection = &rewardCollection
		}
	*/
	/*
		// Get all userLPInfo
		userLPInfo, found := k.GetUserLPInfo(ctx)
		if found {
			genesis.UserLPInfo = &userLPInfo
		}
	*/
	// Get all lpStat
	/*
		lpStat, found := k.GetLpStat(ctx)
		if found {
			genesis.LpStat = &lpStat
		}
	*/
	// this line is used by starport scaffolding # genesis/module/export

	return genesis
}

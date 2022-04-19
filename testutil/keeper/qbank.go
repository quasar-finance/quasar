package keeper

import (
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

const QbankMaccName = types.ModuleName

func (i initializer) QbankKeeper(paramsKeeper paramskeeper.Keeper, bankKeeper bankkeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)
	i.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	qbankKeeper := keeper.NewKeeper(
		i.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
		bankKeeper,
	)
	// Initialize params
	qbankKeeper.SetParams(i.Ctx, types.DefaultParams())

	return qbankKeeper
}

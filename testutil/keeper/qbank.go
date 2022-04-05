package keeper

import (
	"github.com/abag/quasarnode/x/qbank/keeper"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

const QbankMaccName = qbanktypes.ModuleName

func (i initializer) QbankKeeper(paramsKeeper paramskeeper.Keeper, bankKeeper bankkeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(qbanktypes.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(qbanktypes.MemStoreKey)
	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)
	i.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)

	qbankSubspace := paramsKeeper.Subspace(qbanktypes.ModuleName)
	qbankKeeper := keeper.NewKeeper(
		i.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		qbankSubspace,
		bankKeeper,
	)
	// Initialize params
	qbankKeeper.SetParams(i.Ctx, qbanktypes.DefaultParams())

	return qbankKeeper
}

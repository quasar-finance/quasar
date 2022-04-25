package keeper

import (
	epochskeeper "github.com/abag/quasarnode/x/epochs/keeper"
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

const QbankMaccName = types.ModuleName

func (kf KeeperFactory) QbankKeeper(paramsKeeper paramskeeper.Keeper,
	bankKeeper bankkeeper.Keeper,
	epochsKeeper epochskeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
		bankKeeper,
		epochsKeeper,
	)

	return k
}

func (kf KeeperFactory) SetQbankDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

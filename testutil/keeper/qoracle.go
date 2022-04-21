package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (i initializer) QoracleKeeper(paramsKeeper paramskeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)

	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)
	i.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, i.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		i.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
	)

	return *k
}

func (i initializer) SetQoracleDefaultParams(k keeper.Keeper) {
	k.SetParams(i.Ctx, types.DefaultParams())
}

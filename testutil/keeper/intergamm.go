package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/intergamm/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (kf KeeperFactory) IntergammKeeper(paramsKeeper paramskeeper.Keeper, capabilityKeeper capabilitykeeper.Keeper, channelKeeper types.ChannelKeeper, icaControllerKeeper types.ICAControllerKeeper, transferKeeper types.IBCTransferKeeper) *keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)

	scopedKeeper := capabilityKeeper.ScopeToModule(types.ModuleName)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		scopedKeeper,
		channelKeeper,
		icaControllerKeeper,
		transferKeeper,
		paramsSubspace,
	)

	return k
}

func (kf KeeperFactory) SetIntergammDefaultParams(k *keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

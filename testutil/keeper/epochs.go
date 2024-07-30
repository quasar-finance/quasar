package keeper

import (
	storetypes "cosmossdk.io/store/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
)

func (kf KeeperFactory) EpochsKeeper(paramsKeeper paramskeeper.Keeper) *keeper.Keeper {
	storeKey := storetypes.NewKVStoreKey(types.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsKeeper.Subspace(types.ModuleName)
	epochsKeeper := keeper.NewKeeper(kf.EncodingConfig.Marshaler, storeKey)
	epochsKeeper.SetHooks(
		types.NewMultiEpochHooks(),
	)

	return epochsKeeper
}

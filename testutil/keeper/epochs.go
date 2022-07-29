package keeper

import (
	"github.com/quasarlabs/quasarnode/x/epochs/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (kf KeeperFactory) EpochsKeeper(paramsKeeper paramskeeper.Keeper) *keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, kf.DB)

	paramsKeeper.Subspace(types.ModuleName)
	epochsKeeper := keeper.NewKeeper(kf.EncodingConfig.Marshaler, storeKey)
	epochsKeeper.SetHooks(
		types.NewMultiEpochHooks(),
	)

	return epochsKeeper
}

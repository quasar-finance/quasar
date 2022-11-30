package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
)

func (kf KeeperFactory) EpochsKeeper(paramsKeeper paramskeeper.Keeper) *keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsKeeper.Subspace(types.ModuleName)
	epochsKeeper := keeper.NewKeeper(kf.EncodingConfig.Marshaler, storeKey)
	epochsKeeper.SetHooks(
		types.NewMultiEpochHooks(),
	)

	return epochsKeeper
}

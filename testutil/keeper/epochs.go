package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/keeper"
	"github.com/quasarlabs/quasarnode/x/epochs/types"
)

func (f Factory) EpochsKeeper(paramsKeeper paramskeeper.Keeper) *keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	f.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, f.DB)

	paramsKeeper.Subspace(types.ModuleName)
	epochsKeeper := keeper.NewKeeper(f.EncodingConfig.Marshaler, storeKey)
	epochsKeeper.SetHooks(
		types.NewMultiEpochHooks(),
	)

	return epochsKeeper
}

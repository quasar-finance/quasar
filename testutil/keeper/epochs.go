package keeper

import (
	"github.com/abag/quasarnode/x/epochs/keeper"
	"github.com/abag/quasarnode/x/epochs/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (i initializer) EpochsKeeper(paramsKeeper paramskeeper.Keeper) *keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)

	paramsKeeper.Subspace(types.ModuleName)
	epochsKeeper := keeper.NewKeeper(i.EncodingConfig.Marshaler, storeKey)
	epochsKeeper.SetHooks(
		types.NewMultiEpochHooks(),
	)

	return epochsKeeper
}

package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

func TestParamsKeeper(tks *TestKeeperState) (*paramskeeper.Keeper, sdk.Context) {
	storeKey := sdk.NewKVStoreKey(paramstypes.StoreKey)
	transientStoreKey := sdk.NewTransientStoreKey(paramstypes.TStoreKey)
	paramskeeper := paramskeeper.NewKeeper(
		tks.EncodingConfig.Marshaler,
		tks.EncodingConfig.Amino,
		storeKey,
		transientStoreKey,
	)
	tks.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, tks.TestDb)
	tks.StateStore.MountStoreWithDB(transientStoreKey, sdk.StoreTypeTransient, tks.TestDb)

	tks.ParamsKeeper = &paramskeeper

	return &paramskeeper, tks.Ctx
}

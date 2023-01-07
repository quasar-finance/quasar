package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer/types"
)

func (kf KeeperFactory) QTransferKeeper(paramsKeeper paramskeeper.Keeper, accountKeeper authkeeper.AccountKeeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		paramsSubspace,
		accountKeeper,
	)

	return k
}

func (kf KeeperFactory) SetQTransferDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

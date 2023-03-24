package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	"github.com/quasarlabs/quasarnode/x/qtransfer/types"
)

func (f Factory) QTransferKeeper(paramsKeeper paramskeeper.Keeper, accountKeeper authkeeper.AccountKeeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)

	f.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, f.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		f.EncodingConfig.Marshaler,
		storeKey,
		paramsSubspace,
		accountKeeper,
	)

	return k
}

func (f Factory) SetQTransferDefaultParams(k keeper.Keeper) {
	k.SetParams(f.Ctx, types.DefaultParams())
}

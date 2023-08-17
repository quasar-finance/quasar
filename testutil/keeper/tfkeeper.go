package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
	"github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func (kf KeeperFactory) TfKeeper(paramsKeeper paramskeeper.Keeper,
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
	communityPoolKeeper distrkeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	tfKeeper := keeper.NewKeeper(
		storeKey,
		paramsSubspace,
		accountKeeper,
		bankKeeper,
		communityPoolKeeper)

	return tfKeeper
}

func (kf KeeperFactory) SetTFDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

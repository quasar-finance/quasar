package keeper

import (
	storetypes "cosmossdk.io/store/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/quasar-finance/quasar/x/tokenfactory/keeper"
	"github.com/quasar-finance/quasar/x/tokenfactory/types"
)

func (kf KeeperFactory) TfKeeper(paramsKeeper paramskeeper.Keeper,
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
	communityPoolKeeper distrkeeper.Keeper,
) keeper.Keeper {
	storeKey := storetypes.NewKVStoreKey(types.StoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)
	
	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	tfKeeper := keeper.NewKeeper(
		storeKey,
		paramsSubspace,
		nil,
		accountKeeper,
		bankKeeper,
		communityPoolKeeper)

	return tfKeeper
}

func (kf KeeperFactory) SetTFDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

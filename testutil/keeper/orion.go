package keeper

import (
	epochskeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	"github.com/quasarlabs/quasarnode/x/orion/keeper"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbankkeeper "github.com/quasarlabs/quasarnode/x/qbank/keeper"

	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (kf KeeperFactory) OrionKeeper(
	paramsKeeper paramskeeper.Keeper,
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
	qbankKeeper qbankkeeper.Keeper,
	qoracleKeeper qoraclekeeper.Keeper,
	intergammKeeper *intergammkeeper.Keeper,
	epochsKeeper epochskeeper.Keeper,
) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
		accountKeeper,
		bankKeeper,
		qbankKeeper,
		qoracleKeeper,
		intergammKeeper,
		epochsKeeper,
	)

	return *k
}

func (kf KeeperFactory) SetOrionDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	epochskeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	"github.com/quasarlabs/quasarnode/x/qbank/keeper"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	qoraclekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
)

const QbankMaccName = types.ModuleName

func (kf KeeperFactory) QbankKeeper(paramsKeeper paramskeeper.Keeper,
	bankKeeper bankkeeper.Keeper,
	epochsKeeper epochskeeper.Keeper,
	qoracleKeeper qoraclekeeper.Keeper) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memStoreKey, storetypes.StoreTypeMemory, nil)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
		bankKeeper,
		epochsKeeper,
		qoracleKeeper,
	)

	return k
}

func (kf KeeperFactory) SetQbankDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

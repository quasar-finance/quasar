package keeper

import (
	intergammkeeper "github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	qbankkeeper "github.com/abag/quasarnode/x/qbank/keeper"
	qoraclekeeper "github.com/abag/quasarnode/x/qoracle/keeper"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
)

func (i initializer) OrionKeeper(
	paramsKeeper paramskeeper.Keeper,
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
	qbankKeeper qbankkeeper.Keeper,
	qoracleKeeper qoraclekeeper.Keeper,
	intergammKeeper intergammkeeper.Keeper,
) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)

	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)
	i.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, i.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		i.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		paramsSubspace,
		accountKeeper,
		bankKeeper,
		qbankKeeper,
		qoracleKeeper,
		intergammKeeper,
	)

	return *k
}

func (i initializer) SetOrionDefaultParams(k keeper.Keeper) {
	k.SetParams(i.Ctx, types.DefaultParams())
}

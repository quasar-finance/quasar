package keeper

import (
	"github.com/abag/quasarnode/app"
	"github.com/cosmos/cosmos-sdk/store"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmdb "github.com/tendermint/tm-db"
)

type initializer struct {
	DB             *tmdb.MemDB
	StateStore     store.CommitMultiStore
	Ctx            sdk.Context
	EncodingConfig cosmoscmd.EncodingConfig
}

func newInitializer() initializer {
	logger := log.TestingLogger()
	logger.Debug("creating TestKeeperState")

	db := tmdb.NewMemDB()
	stateStore := store.NewCommitMultiStore(db)

	ctx := sdk.NewContext(stateStore, tmproto.Header{}, false, logger)
	encodingConfig := cosmoscmd.MakeEncodingConfig(app.ModuleBasics)

	return initializer{
		DB:             db,
		StateStore:     stateStore,
		Ctx:            ctx,
		EncodingConfig: encodingConfig,
	}
}

func (i initializer) ParamsKeeper() paramskeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(paramstypes.StoreKey)
	transientStoreKey := sdk.NewTransientStoreKey(paramstypes.TStoreKey)
	paramskeeper := paramskeeper.NewKeeper(i.EncodingConfig.Marshaler, i.EncodingConfig.Amino, storeKey, transientStoreKey)

	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)
	i.StateStore.MountStoreWithDB(transientStoreKey, sdk.StoreTypeTransient, i.DB)

	return paramskeeper
}

func (i initializer) AccountKeeper(paramsKeeper paramskeeper.Keeper, maccPerms map[string][]string) authkeeper.AccountKeeper {
	storeKey := sdk.NewKVStoreKey(authtypes.StoreKey)
	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)

	subspace := paramsKeeper.Subspace(authtypes.ModuleName)
	accountKeeper := authkeeper.NewAccountKeeper(
		i.EncodingConfig.Marshaler, storeKey, subspace, authtypes.ProtoBaseAccount, maccPerms,
	)

	return accountKeeper
}

func (i initializer) BankKeeper(paramsKeeper paramskeeper.Keeper, accountKeeper authkeeper.AccountKeeper, blockedMaccAddresses map[string]bool) bankkeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(banktypes.StoreKey)
	i.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, i.DB)

	subspace := paramsKeeper.Subspace(banktypes.ModuleName)
	bankKeeper := bankkeeper.NewBaseKeeper(
		i.EncodingConfig.Marshaler, storeKey, accountKeeper, subspace, blockedMaccAddresses,
	)

	return bankKeeper
}

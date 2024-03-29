package keeper

import (
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"testing"

	"github.com/cosmos/cosmos-sdk/codec"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/store"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	typesparams "github.com/cosmos/cosmos-sdk/x/params/types"
	"github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	"github.com/quasarlabs/quasarnode/x/qvesting/types"
	"github.com/stretchr/testify/require"
	"github.com/tendermint/tendermint/libs/log"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	tmdb "github.com/tendermint/tm-db"
)

func QVestingKeeper(t testing.TB) (*keeper.Keeper, sdk.Context) {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)

	db := tmdb.NewMemDB()
	stateStore := store.NewCommitMultiStore(db)
	stateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, db)
	stateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)
	require.NoError(t, stateStore.LoadLatestVersion())

	registry := codectypes.NewInterfaceRegistry()
	cdc := codec.NewProtoCodec(registry)

	paramsSubspace := typesparams.NewSubspace(cdc,
		types.Amino,
		storeKey,
		memStoreKey,
		"QVestingParams",
	)
	k := keeper.NewKeeper(
		cdc,
		storeKey,
		memStoreKey,
		paramsSubspace,
		nil,
		nil,
	)

	ctx := sdk.NewContext(stateStore, tmproto.Header{}, false, log.NewNopLogger())

	// Initialize params
	k.SetParams(ctx, types.DefaultParams())

	return k, ctx
}

func (kf KeeperFactory) QVestingKeeper(paramsKeeper paramskeeper.Keeper, accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper) keeper.Keeper {

	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	//storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)

	//db := tmdb.NewMemDB()
	//stateStore := store.NewCommitMultiStore(db)
	//stateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, db)
	//stateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)
	//require.NoError(t, stateStore.LoadLatestVersion())

	registry := codectypes.NewInterfaceRegistry()
	cdc := codec.NewProtoCodec(registry)

	//paramsSubspace := typesparams.NewSubspace(cdc,
	//	types.Amino,
	//	storeKey,
	//	memStoreKey,
	//	"QVestingParams",
	//)
	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)

	k := keeper.NewKeeper(
		cdc,
		storeKey,
		memStoreKey,
		paramsSubspace,
		accountKeeper,
		bankKeeper,
	)

	// Initialize params
	// k.SetParams(ctx, types.DefaultParams())

	return *k
}

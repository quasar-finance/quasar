package keeper

import (
	storetypes "cosmossdk.io/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	capabilitykeeper "github.com/cosmos/ibc-go/modules/capability/keeper"
	capabilitytypes "github.com/cosmos/ibc-go/modules/capability/types"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
)

const (
	AccountAddressPrefix = "quasar"
)

func (kf KeeperFactory) ParamsKeeper() paramskeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(paramstypes.StoreKey)
	transientStoreKey := sdk.NewTransientStoreKey(paramstypes.TStoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(transientStoreKey, storetypes.StoreTypeTransient, kf.DB)

	paramsKeeper := paramskeeper.NewKeeper(kf.EncodingConfig.Marshaler, kf.EncodingConfig.Amino, storeKey, transientStoreKey)

	return paramsKeeper
}

func (kf KeeperFactory) AccountKeeper(paramsKeeper paramskeeper.Keeper, maccPerms map[string][]string) authkeeper.AccountKeeper {
	storeKey := sdk.NewKVStoreKey(authtypes.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	accountKeeper := authkeeper.NewAccountKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		authtypes.ProtoBaseAccount,
		maccPerms,
		AccountAddressPrefix,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	return accountKeeper
}

func (kf KeeperFactory) BankKeeper(paramsKeeper paramskeeper.Keeper, accountKeeper authkeeper.AccountKeeper, blockedMaccAddresses map[string]bool) bankkeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(banktypes.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	bankKeeper := bankkeeper.NewBaseKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		accountKeeper,
		blockedMaccAddresses,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	return bankKeeper
}

func (kf KeeperFactory) CapabilityKeeper() capabilitykeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(capabilitytypes.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(capabilitytypes.MemStoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memStoreKey, storetypes.StoreTypeMemory, nil)

	capabilityKeeper := capabilitykeeper.NewKeeper(
		kf.EncodingConfig.Marshaler, storeKey, memStoreKey,
	)

	return *capabilityKeeper
}

func (kf KeeperFactory) StakingKeeper(
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
) stakingkeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(stakingtypes.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	stakingKeeper := stakingkeeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		accountKeeper,
		bankKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	return *stakingKeeper
}

func (kf KeeperFactory) DistributionKeeper(
	accountKeeper authkeeper.AccountKeeper,
	bankKeeper bankkeeper.Keeper,
	stakingKeeper stakingkeeper.Keeper,
	feeCollectorName string,
) distrkeeper.Keeper {

	storeKey := sdk.NewKVStoreKey(distrtypes.StoreKey)
	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	disrKeeper := distrkeeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		accountKeeper,
		bankKeeper,
		stakingKeeper,
		feeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	return disrKeeper
}

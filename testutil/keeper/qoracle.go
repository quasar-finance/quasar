package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	qbandkeeper "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/keeper"
	qbandtypes "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (kf KeeperFactory) QbandchainKeeper(
	paramsKeeper paramskeeper.Keeper,
	clientKeeper types.ClientKeeper,
	ics4Wrapper porttypes.ICS4Wrapper,
	channelKeeper types.ChannelKeeper,
	portKeeper types.PortKeeper,
	scopedKeeper capabilitykeeper.ScopedKeeper,
	qoracleKeeper qbandtypes.QOracle,
) qbandkeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(qbandtypes.StoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(qbandtypes.SubModuleName)
	k := qbandkeeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		paramsSubspace,
		clientKeeper,
		ics4Wrapper,
		channelKeeper,
		portKeeper,
		scopedKeeper,
		qoracleKeeper,
	)

	return k
}

func (kf KeeperFactory) SetQbandchainDefaultParams(k qbandkeeper.Keeper) {
	k.SetParams(kf.Ctx, qbandtypes.DefaultParams())
}

func (kf KeeperFactory) QosmosisKeeper(
	paramsKeeper paramskeeper.Keeper,
	authority string,
	clientKeeper types.ClientKeeper,
	ics4Wrapper porttypes.ICS4Wrapper,
	channelKeeper types.ChannelKeeper,
	portKeeper types.PortKeeper,
	scopedKeeper capabilitykeeper.ScopedKeeper,
	qoracleKeeper qosmotypes.QOracle,
) qosmokeeper.Keeper {
	storeKey := sdk.NewKVStoreKey(qosmotypes.StoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(qosmotypes.SubModuleName)
	k := qosmokeeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		paramsSubspace,
		authority,
		clientKeeper,
		ics4Wrapper,
		channelKeeper,
		portKeeper,
		scopedKeeper,
		qoracleKeeper,
	)

	return k
}

func (kf KeeperFactory) SetQosmosisDefaultParams(k qosmokeeper.Keeper) {
	k.SetParams(kf.Ctx, qosmotypes.DefaultParams())
}

func (kf KeeperFactory) QoracleKeeper(paramsKeeper paramskeeper.Keeper, authority string) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	tKey := sdk.NewTransientStoreKey(types.TStoreKey)

	kf.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, kf.DB)
	kf.StateStore.MountStoreWithDB(memKey, storetypes.StoreTypeMemory, kf.DB)
	kf.StateStore.MountStoreWithDB(tKey, storetypes.StoreTypeTransient, kf.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		kf.EncodingConfig.Marshaler,
		storeKey,
		memKey,
		tKey,
		paramsSubspace,
		authority,
	)

	return k
}

func (kf KeeperFactory) SetQoracleDefaultParams(k keeper.Keeper) {
	k.SetParams(kf.Ctx, types.DefaultParams())
}

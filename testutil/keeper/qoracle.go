package keeper

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func (f Factory) QosmosisKeeper(
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

	f.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, f.DB)

	paramsSubspace := paramsKeeper.Subspace(qosmotypes.SubModuleName)
	k := qosmokeeper.NewKeeper(
		f.EncodingConfig.Marshaler,
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

func (f Factory) SetQosmosisDefaultParams(k qosmokeeper.Keeper) {
	k.SetParams(f.Ctx, qosmotypes.DefaultParams())
}

func (f Factory) QoracleKeeper(paramsKeeper paramskeeper.Keeper, authority string) keeper.Keeper {
	storeKey := sdk.NewKVStoreKey(types.StoreKey)
	memKey := storetypes.NewMemoryStoreKey(types.MemStoreKey)
	tKey := sdk.NewTransientStoreKey(types.TStoreKey)

	f.StateStore.MountStoreWithDB(storeKey, storetypes.StoreTypeIAVL, f.DB)
	f.StateStore.MountStoreWithDB(memKey, storetypes.StoreTypeMemory, f.DB)
	f.StateStore.MountStoreWithDB(tKey, storetypes.StoreTypeTransient, f.DB)

	paramsSubspace := paramsKeeper.Subspace(types.ModuleName)
	k := keeper.NewKeeper(
		f.EncodingConfig.Marshaler,
		storeKey,
		memKey,
		tKey,
		paramsSubspace,
		authority,
	)

	return k
}

func (f Factory) SetQoracleDefaultParams(k keeper.Keeper) {
	k.SetParams(f.Ctx, types.DefaultParams())
}

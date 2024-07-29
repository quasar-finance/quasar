package upgrades

import (
	"github.com/cosmos/cosmos-sdk/store/prefix"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	"github.com/quasarlabs/quasarnode/app/keepers"
)

func Upgrades() []Upgrade {
	return []Upgrade{
		{UpgradeName: V030000UpgradeName, CreateUpgradeHandler: V030000UpgradeHandler},
	}
}

// empty upgrade handler
func NoOpHandler(
	mm *module.Manager,
	configurator module.Configurator,
	_ *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx sdk.Context, _ upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

// V030000UpgradeHandler is for v3.0.0 upgrade , that removes qtransfer, qoracle and qvesting module-states
func V030000UpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx sdk.Context, _ upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		modulesToRemove := []string{"qtransfer", "qoracle", "qvesting"}

		// Iterate over each module
		for _, moduleName := range modulesToRemove {
			storeKey := keepers.GetKey(moduleName)

			// Create a prefix store using the store key
			store := ctx.KVStore(storeKey)
			prefixStore := prefix.NewStore(store, []byte{})

			// Iterate over all key-value pairs in the store and delete them
			iterator := prefixStore.Iterator(nil, nil)
			defer func(iterator storetypes.Iterator) {
				err := iterator.Close()
				if err != nil {
					panic(err)
				}
			}(iterator)

			for ; iterator.Valid(); iterator.Next() {
				prefixStore.Delete(iterator.Key())
			}
		}
		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

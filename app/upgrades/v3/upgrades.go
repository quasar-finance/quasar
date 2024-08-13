package v3

import (
	"context"

	"cosmossdk.io/math"
	"cosmossdk.io/store/prefix"
	storetypes "cosmossdk.io/store/types"
	upgradetypes "cosmossdk.io/x/upgrade/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	pfmtypes "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/types"
	ratelimittypes "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/types"
	"github.com/quasar-finance/quasar/app/keepers"
	feemarkettypes "github.com/skip-mev/feemarket/x/feemarket/types"
)

func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(c context.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		modulesToRemove := []string{"qtransfer", "qoracle", "qvesting"}
		ctx := sdk.UnwrapSDKContext(c)
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

		// Set rate-limit params
		keepers.RatelimitKeeper.SetParams(ctx, ratelimittypes.DefaultParams())
		// Set pfm params
		err := keepers.PFMRouterKeeper.SetParams(ctx, pfmtypes.DefaultParams())
		if err != nil {
			panic(err)
		}
		// fee market params
		// TODO: change values from default after discussion
		feemarketParams := feemarkettypes.DefaultParams()
		feemarketParams.MinBaseGasPrice = math.LegacyMustNewDecFromStr("0.10000000000000000")
		feemarketParams.MaxBlockUtilization = uint64(120000000)
		feemarketParams.FeeDenom = "uqsr"
		err = keepers.FeeMarketKeeper.SetParams(ctx, feemarketParams)
		if err != nil {
			panic(err)
		}

		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

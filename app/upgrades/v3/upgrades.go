package v3

import (
	"context"
	"fmt"

	errorsmod "cosmossdk.io/errors"
	"cosmossdk.io/math"
	storetypes "cosmossdk.io/store/types"
	upgradetypes "cosmossdk.io/x/upgrade/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	pfmtypes "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/types"
	ratelimittypes "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/types"
	"github.com/quasar-finance/quasar/app/keepers"
	appparams "github.com/quasar-finance/quasar/app/params"
	feemarketkeeper "github.com/skip-mev/feemarket/x/feemarket/keeper"
	feemarkettypes "github.com/skip-mev/feemarket/x/feemarket/types"
)

// CreateUpgradeHandler runs migrations and param changes needed for upgrade.
// Note : Always use RunMigrations before setting new params
// as RunMigrations calls InitGenesis with default params
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

			// Access the store directly using the store key
			store := ctx.KVStore(storeKey)

			// Iterate over all key-value pairs in the store and delete them
			iterator := store.Iterator(nil, nil)
			defer func(iterator storetypes.Iterator) {
				err := iterator.Close()
				if err != nil {
					panic(err)
				}
			}(iterator)

			for ; iterator.Valid(); iterator.Next() {
				store.Delete(iterator.Key())
			}
		}

		vm, err := mm.RunMigrations(ctx, configurator, fromVM)
		if err != nil {
			return vm, err
		}

		// Set rate-limit params
		keepers.RatelimitKeeper.SetParams(ctx, ratelimittypes.DefaultParams())
		// Set pfm params
		err = keepers.PFMRouterKeeper.SetParams(ctx, pfmtypes.DefaultParams())
		if err != nil {
			return nil, err
		}
		// fee market params
		// TODO: change values from default after discussion
		ctx.Logger().Info("Setting dynamicfees/feemarket params...")
		err = setFeeMarketParams(ctx, keepers.FeeMarketKeeper)
		if err != nil {
			return nil, err
		}

		ctx.Logger().Info(fmt.Sprintf("Migration {%s} applied", UpgradeName))
		return vm, nil
	}
}

func setFeeMarketParams(ctx sdk.Context, feemarketKeeper *feemarketkeeper.Keeper) error {
	feemarketParams := feemarkettypes.DefaultParams()
	feemarketParams.MinBaseGasPrice = math.LegacyMustNewDecFromStr("0.10000000000000000")
	feemarketParams.MaxBlockUtilization = uint64(120_000_000)
	feemarketParams.FeeDenom = appparams.DefaultBondDenom
	feemarketParams.Enabled = true
	feemarketParams.DistributeFees = true

	feemarketState := feemarkettypes.NewState(feemarketParams.Window, feemarketParams.MinBaseGasPrice, feemarketParams.MinLearningRate)
	err := feemarketKeeper.SetParams(ctx, feemarketParams)
	if err != nil {
		return errorsmod.Wrap(err, "failed to set feemarket params")
	}
	err = feemarketKeeper.SetState(ctx, feemarketState)
	if err != nil {
		return errorsmod.Wrap(err, "failed to set feemarket state")
	}

	return nil
}

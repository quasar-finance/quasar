package v2

import (
	"context"

	upgradetypes "cosmossdk.io/x/upgrade/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/quasar-finance/quasar/app/keepers"
)

// qosmotypes "github.com/quasar-finance/quasar/x/qoracle/osmosis/types"

// TODO -SDK 50 - This files does not need to be present any more. However we will loose some old Handlers,
// RECHECK, what to do as some of the modules like qoracle is gone.

// TODO - SDK 50 - This is a dummy Handler for now.
func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx context.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {

		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

/*
func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	bpm upgrades.BaseAppParamManager,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		fromVM[govtypes.ModuleName] = 1
		baseAppLegacySS := keepers.ParamsKeeper.Subspace(baseapp.Paramspace).WithKeyTable(paramstypes.ConsensusParamsKeyTable())

		// https://github.com/cosmos/cosmos-sdk/pull/12363/files
		// Set param key table for params module migration
		for _, subspace := range keepers.ParamsKeeper.GetSubspaces() {
			subspace := subspace

			var keyTable paramstypes.KeyTable
			switch subspace.Name() {
			// sdk
			case authtypes.ModuleName:
				keyTable = authtypes.ParamKeyTable() //nolint:staticcheck
			case banktypes.ModuleName:
				keyTable = banktypes.ParamKeyTable() //nolint:staticcheck
			case stakingtypes.ModuleName:
				keyTable = stakingtypes.ParamKeyTable() //nolint:staticcheck
			case minttypes.ModuleName:
				keyTable = minttypes.ParamKeyTable() //nolint:staticcheck
			case distrtypes.ModuleName:
				keyTable = distrtypes.ParamKeyTable() //nolint:staticcheck
			case slashingtypes.ModuleName:
				keyTable = slashingtypes.ParamKeyTable() //nolint:staticcheck
			case govtypes.ModuleName:
				keyTable = govv1.ParamKeyTable() //nolint:staticcheck
			case crisistypes.ModuleName:
				keyTable = crisistypes.ParamKeyTable() //nolint:staticcheck

			// ibc types
			case ibctransfertypes.ModuleName:
				keyTable = ibctransfertypes.ParamKeyTable() //nolint:staticcheck
			case icahosttypes.SubModuleName:
				keyTable = icahosttypes.ParamKeyTable() //nolint:staticcheck
			case icacontrollertypes.SubModuleName:
				keyTable = icacontrollertypes.ParamKeyTable() //nolint:staticcheck
			case icqtypes.ModuleName:
				keyTable = icqtypes.ParamKeyTable() //nolint:staticcheck

			// wasm
			case wasmtypes.ModuleName:
				keyTable = wasmtypes.ParamKeyTable() //nolint:staticcheck

			// quasar
			//case qoraclemoduletypes.ModuleName:
			// 	keyTable = qoraclemoduletypes.ParamKeyTable()
			case qosmotypes.SubModuleName:
				keyTable = qosmotypes.ParamKeyTable()
			case qtransfertypes.ModuleName:
				keyTable = qtransfertypes.ParamKeyTable()
			case tftypes.ModuleName:
				keyTable = tftypes.ParamKeyTable()
			// qvesting does not have params

			default:
				continue
			}

			if !subspace.HasKeyTable() {
				subspace.WithKeyTable(keyTable)
			}
		}

		// Migrate Tendermint consensus parameters from x/params module to a deprecated x/consensus module.
		// The old params module is required to still be imported in your app.go in order to handle this migration.
		baseapp.MigrateParams(ctx, baseAppLegacySS, keepers.ConsensusParamsKeeper)

		migrations, err := mm.RunMigrations(ctx, configurator, fromVM)
		if err != nil {
			return nil, err
		}

		return migrations, nil
	}
}
*/

package v0

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	qvestingkeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	qvestingtypes "github.com/quasarlabs/quasarnode/x/qvesting/types"

	"github.com/quasarlabs/quasarnode/app/keepers"
	"github.com/quasarlabs/quasarnode/app/upgrades"
)

func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	bpm upgrades.BaseAppParamManager,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		setQVestingParams(ctx, &keepers.QVestingKeeper)

		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

func setQVestingParams(ctx sdk.Context, qvestingKeeper *qvestingkeeper.Keeper) {
	qvestingParams := qvestingtypes.DefaultParams()
	qvestingKeeper.SetParams(ctx, qvestingParams)
}

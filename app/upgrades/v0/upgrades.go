package v0

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	"github.com/quasarlabs/quasarnode/app/keepers"
	"github.com/quasarlabs/quasarnode/app/upgrades"
	qvestingkeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	qvestingtypes "github.com/quasarlabs/quasarnode/x/qvesting/types"
	tfkeeper "github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
	tftypes "github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

func CreateUpgradeHandler(
	mm *module.Manager,
	configurator module.Configurator,
	bpm upgrades.BaseAppParamManager,
	keepers *keepers.AppKeepers,
) upgradetypes.UpgradeHandler {
	return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		setQVestingParams(ctx, &keepers.QVestingKeeper)
		setTfParams(ctx, &keepers.TfKeeper)

		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

func setQVestingParams(ctx sdk.Context, qvestingKeeper *qvestingkeeper.Keeper) {
	qvestingParams := qvestingtypes.DefaultParams()
	qvestingKeeper.SetParams(ctx, qvestingParams)
}

func setTfParams(ctx sdk.Context, tfKeeper *tfkeeper.Keeper) {
	tfParams := tftypes.DefaultParams()
	tfKeeper.SetParams(ctx, tfParams)
}

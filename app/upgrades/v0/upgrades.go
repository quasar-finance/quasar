package v0

import (
	"context"

	upgradetypes "cosmossdk.io/x/upgrade/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
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
	return func(ctx context.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
		setQVestingParams(ctx, &keepers.QVestingKeeper)
		setTfParams(ctx, &keepers.TfKeeper)

		return mm.RunMigrations(ctx, configurator, fromVM)
	}
}

func setQVestingParams(ctx context.Context, qvestingKeeper *qvestingkeeper.Keeper) {
	sdkCtx := sdk.UnwrapSDKContext(ctx)
	qvestingParams := qvestingtypes.DefaultParams()
	qvestingKeeper.SetParams(sdkCtx, qvestingParams)
}

func setTfParams(ctx context.Context, tfKeeper *tfkeeper.Keeper) {
	sdkCtx := sdk.UnwrapSDKContext(ctx)
	tfParams := tftypes.DefaultParams()
	tfKeeper.SetParams(sdkCtx, tfParams)
}

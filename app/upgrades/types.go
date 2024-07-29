package upgrades

import (
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	"github.com/quasarlabs/quasarnode/app/keepers"
)

const (
	ProdChainID = "quasar-1"
	TestChainID = "quasar-test-1"

	// upgrade name consts: vMMmmppUpgradeName (M=Major, m=minor, p=patch).
	V030000UpgradeName = "v3.0.0"
)

// Upgrade defines a struct containing necessary fields that a SoftwareUpgradeProposal
// must have written, in order for the state migration to go smoothly.
// An upgrade must implement this struct, and then set it in the app.go.
// The app.go will then define the handler.
type Upgrade struct {
	// Upgrade version name, for the upgrade handler, e.g. `v7`
	UpgradeName string

	// CreateUpgradeHandler defines the function that creates an upgrade handler
	CreateUpgradeHandler func(*module.Manager, module.Configurator, *keepers.AppKeepers) upgradetypes.UpgradeHandler

	// Store upgrades, should be used for any new modules introduced, new modules deleted, or store names renamed.
	StoreUpgrades storetypes.StoreUpgrades
}

// todo before release
// func isMainnet(ctx sdk.Context) bool {
//	return ctx.ChainID() == ProdChainID
//}
//
//func isTestnet(ctx sdk.Context) bool {
//	return ctx.ChainID() == TestChainID
//}

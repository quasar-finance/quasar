package v3

import (
	store "cosmossdk.io/store/types"
	circuittypes "cosmossdk.io/x/circuit/types"
	"github.com/quasar-finance/quasar/app/upgrades"
)

// UpgradeName defines the on-chain upgrade name for the Quasar chain v2.0.0 upgrade.
const UpgradeName = "v3"

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added: []string{
			circuittypes.ModuleName,
		},
		Deleted: []string{},
	},
}

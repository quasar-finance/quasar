package v4

import (
	storetypes "cosmossdk.io/store/types"
	"github.com/quasar-finance/quasar/app/upgrades"
	marketmaptypes "github.com/skip-mev/slinky/x/marketmap/types"
	oracletypes "github.com/skip-mev/slinky/x/oracle/types"
)

const (
	UpgradeName                = "v4"
	MarketMapAuthorityMultisig = "" //TODO: add
	DecimalAdjustment          = 1_000_000_000_000
)

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: storetypes.StoreUpgrades{
		Added: []string{
			marketmaptypes.StoreKey,
			oracletypes.StoreKey,
		},
	},
}

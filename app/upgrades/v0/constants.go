package v0

import (
	store "github.com/cosmos/cosmos-sdk/store/types"
	"github.com/quasarlabs/quasarnode/app/upgrades"
	qvestingtypes "github.com/quasarlabs/quasarnode/x/qvesting/types"
	tftypes "github.com/quasarlabs/quasarnode/x/tokenfactory/types"
)

// UpgradeName defines the on-chain upgrade name for the Osmosis v15 upgrade.
const UpgradeName = "v0.1.1"

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added:   []string{qvestingtypes.StoreKey, tftypes.StoreKey},
		Deleted: []string{},
	},
}

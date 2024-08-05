package v3

import (
	store "cosmossdk.io/store/types"
	consensustypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	icqtypes "github.com/cosmos/ibc-apps/modules/async-icq/v8/types"
	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"
	"github.com/quasar-finance/quasar/app/upgrades"
)

// UpgradeName defines the on-chain upgrade name for the Quasar chain v2.0.0 upgrade.
const UpgradeName = "v3"

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added: []string{ // v47 modules
			crisistypes.StoreKey,
			consensustypes.StoreKey,
			ibcwasmtypes.StoreKey,
			icqtypes.StoreKey,
			// todo add circuit types for wasm migration
		},
		Deleted: []string{},
	},
}

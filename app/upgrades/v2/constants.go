package v2

import (
	store "github.com/cosmos/cosmos-sdk/store/types"
	consensustypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	icqtypes "github.com/cosmos/ibc-apps/modules/async-icq/v7/types"
	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"
	"github.com/quasarlabs/quasarnode/app/upgrades"
)

// UpgradeName defines the on-chain upgrade name for the Quasar chain v2.0.0 upgrade.
const UpgradeName = "v2"

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added: []string{ // v47 modules
			crisistypes.StoreKey,
			consensustypes.StoreKey,
			ibcwasmtypes.StoreKey,
			icqtypes.StoreKey,
		},
		Deleted: []string{},
	},
}

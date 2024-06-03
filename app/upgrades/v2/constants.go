package v2

import (
	store "github.com/cosmos/cosmos-sdk/store/types"
	consensustypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	feegranttypes "github.com/cosmos/cosmos-sdk/x/feegrant"
	"github.com/quasarlabs/quasarnode/app/upgrades"
)

// UpgradeName defines the on-chain upgrade name for the Quasar chain v2.0.0 upgrade.
const UpgradeName = "v2"

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added: []string{ // v47 modules
			crisistypes.ModuleName,
			consensustypes.ModuleName,
			feegranttypes.ModuleName,
		},
		Deleted: []string{},
	},
}

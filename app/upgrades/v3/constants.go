package v3

import (
	store "cosmossdk.io/store/types"
	circuittypes "cosmossdk.io/x/circuit/types"
	pfmtypes "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/types"
	ibchookstypes "github.com/cosmos/ibc-apps/modules/ibc-hooks/v8/types"
	ratelimittypes "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/types"
	"github.com/quasar-finance/quasar/app/upgrades"
	feemarkettypes "github.com/skip-mev/feemarket/x/feemarket/types"
)

// UpgradeName defines the on-chain upgrade name for the Quasar chain v2.0.0 upgrade.
const (
	UpgradeName = "v3"
)

var Upgrade = upgrades.Upgrade{
	UpgradeName:          UpgradeName,
	CreateUpgradeHandler: CreateUpgradeHandler,
	StoreUpgrades: store.StoreUpgrades{
		Added: []string{
			circuittypes.StoreKey, feemarkettypes.StoreKey, ibchookstypes.StoreKey, pfmtypes.StoreKey, ratelimittypes.StoreKey,
		},
		Deleted: []string{},
	},
}

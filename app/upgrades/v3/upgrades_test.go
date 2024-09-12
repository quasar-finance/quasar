package v3_test

import (
	"fmt"
	"testing"
	"time"

	"cosmossdk.io/core/appmodule"
	"cosmossdk.io/core/header"
	"cosmossdk.io/math"
	"cosmossdk.io/store/prefix"
	storetypes "cosmossdk.io/store/types"
	"cosmossdk.io/x/upgrade"
	upgradetypes "cosmossdk.io/x/upgrade/types"
	addresscodec "github.com/cosmos/cosmos-sdk/codec/address"
	pfmtypes "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/types"
	ratelimittypes "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/types"
	"github.com/quasar-finance/quasar/app/apptesting"
	v3 "github.com/quasar-finance/quasar/app/upgrades/v3"
	feemarkettypes "github.com/skip-mev/feemarket/x/feemarket/types"
	"github.com/stretchr/testify/suite"
)

const (
	v3UpgradeHeight = int64(5)
)

type UpgradeTestSuite struct {
	preModule appmodule.HasPreBlocker
	apptesting.KeeperTestHelper
}

func TestUpgradeTestSuite(t *testing.T) {
	suite.Run(t, new(UpgradeTestSuite))
}

func (s *UpgradeTestSuite) PreBlockerSetup() {
	s.preModule = upgrade.NewAppModule(s.App.UpgradeKeeper, addresscodec.NewBech32Codec("osmo"))
}
func (s *UpgradeTestSuite) TestUpgrade() {
	s.Setup()
	s.PreBlockerSetup()

	// Pre-upgrade setup: Insert dummy data into the stores to be deleted
	ctx := s.Ctx
	keepers := s.App.AppKeepers

	modulesToRemove := []string{} // as now the module keys are deprecated and cleaned up post v3 upgrade this fails otherwise
	for _, moduleName := range modulesToRemove {
		storeKey := keepers.GetKey(moduleName)
		store := ctx.KVStore(storeKey)
		prefixStore := prefix.NewStore(store, []byte{})

		// Insert a dummy key-value pair
		prefixStore.Set([]byte("dummyKey"), []byte("dummyValue"))
	}

	// Simulate the upgrade
	dummyUpgrade(s)
	s.Require().NotPanics(func() {
		_, err := s.preModule.PreBlock(s.Ctx)
		s.Require().NoError(err)
	})

	// Post-upgrade assertions
	for _, moduleName := range modulesToRemove {
		storeKey := keepers.GetKey(moduleName)
		store := ctx.KVStore(storeKey)

		// Iterate over all key-value pairs in the store to verify it's empty
		iterator := store.Iterator(nil, nil)
		defer func(iterator storetypes.Iterator) {
			err := iterator.Close()
			if err != nil {
				panic(err)
			}
		}(iterator)

		// Check if the iterator is valid, which would mean the store is not empty
		s.Require().False(iterator.Valid(), fmt.Sprintf("Store for module %s is not empty", moduleName))
	}

	// Assert that the rate-limit params were set correctly
	rateLimitParams := keepers.RatelimitKeeper.GetParams(ctx)
	s.Require().Equal(ratelimittypes.DefaultParams(), rateLimitParams, "Rate limit params do not match default params after upgrade")

	// Assert that the PFM params were set correctly
	pfmParams := keepers.PFMRouterKeeper.GetParams(ctx)
	s.Require().Equal(pfmtypes.DefaultParams(), pfmParams, "PFM params do not match default params after upgrade")

	// Assert that the fee market params were set correctly
	feemarketParams, err := keepers.FeeMarketKeeper.GetParams(ctx)
	s.Require().NoError(err, "Error getting feemarket params after upgrade")
	expectedFeeMarketParams := feemarkettypes.DefaultParams()
	expectedFeeMarketParams.Alpha = math.LegacyMustNewDecFromStr("0.003000000000000000")
	expectedFeeMarketParams.Beta = math.LegacyMustNewDecFromStr("0.980000000000000000")
	expectedFeeMarketParams.Delta = math.LegacyMustNewDecFromStr("0.001500000000000000")
	expectedFeeMarketParams.DistributeFees = true
	expectedFeeMarketParams.Enabled = true
	expectedFeeMarketParams.FeeDenom = "uqsr"
	expectedFeeMarketParams.Gamma = math.LegacyMustNewDecFromStr("0.008000000000000000")
	expectedFeeMarketParams.MaxBlockUtilization = uint64(120000000)
	expectedFeeMarketParams.MaxLearningRate = math.LegacyMustNewDecFromStr("0.125000000000000000")
	expectedFeeMarketParams.MinBaseGasPrice = math.LegacyMustNewDecFromStr("0.100000000000000000")
	expectedFeeMarketParams.MinLearningRate = math.LegacyMustNewDecFromStr("0.075000000000000000")
	expectedFeeMarketParams.Window = uint64(7)
	s.Require().Equal(expectedFeeMarketParams, feemarketParams, "Fee market params do not match expected values after upgrade")
}

func dummyUpgrade(s *UpgradeTestSuite) {
	s.Ctx = s.Ctx.WithBlockHeight(v3UpgradeHeight - 1)
	plan := upgradetypes.Plan{Name: v3.Upgrade.UpgradeName, Height: v3UpgradeHeight}
	err := s.App.UpgradeKeeper.ScheduleUpgrade(s.Ctx, plan)
	s.Require().NoError(err)
	_, err = s.App.UpgradeKeeper.GetUpgradePlan(s.Ctx)
	s.Require().NoError(err)

	s.Ctx = s.Ctx.WithHeaderInfo(header.Info{Height: v3UpgradeHeight, Time: s.Ctx.BlockTime().Add(time.Second)}).WithBlockHeight(v3UpgradeHeight)
}

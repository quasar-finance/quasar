package v4_test

import (
	"sort"
	"testing"
	"time"

	"cosmossdk.io/core/appmodule"
	"cosmossdk.io/core/header"
	"cosmossdk.io/x/upgrade"
	upgradetypes "cosmossdk.io/x/upgrade/types"
	comettypes "github.com/cometbft/cometbft/proto/tendermint/types"
	addresscodec "github.com/cosmos/cosmos-sdk/codec/address"
	"github.com/cosmos/cosmos-sdk/x/consensus/types"
	"github.com/quasar-finance/quasar/app/apptesting"
	v4 "github.com/quasar-finance/quasar/app/upgrades/v4"
	slinkyconstants "github.com/skip-mev/slinky/cmd/constants/marketmaps"
	slinkytypes "github.com/skip-mev/slinky/pkg/types"
	marketmaptypes "github.com/skip-mev/slinky/x/marketmap/types"
	"github.com/stretchr/testify/suite"
)

const (
	v4UpgradeHeight = int64(100)
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

func (s *UpgradeTestSuite) TestOracleUpgrade() {
	s.Setup()
	s.PreBlockerSetup()
	keepers := s.App.AppKeepers

	oldParams, err := s.App.ConsensusParamsKeeper.Params(s.Ctx, &types.QueryParamsRequest{})
	s.Require().NoError(err)
	oldParams.Params.Version = &comettypes.VersionParams{App: 0}
	// we need to properly set consensus params for tests or we get a panic
	s.Require().NoError(s.App.ConsensusParamsKeeper.ParamsStore.Set(s.Ctx, *oldParams.Params))

	markets := slinkyconstants.CoreMarketMap.Markets
	s.Require().NoError(err)
	// Simulate the upgrade
	dummyUpgrade(s)
	s.Require().NotPanics(func() {
		_, err := s.preModule.PreBlock(s.Ctx)
		s.Require().NoError(err)

	})

	params, err := keepers.MarketmapKeeper.GetParams(s.Ctx)
	s.Require().NoError(err)
	s.Require().Equal(params.MarketAuthorities[0], "quasar10d07y265gmmuvt4z0w9aw880jnsr700j5pfcmk")
	s.Require().Equal(params.Admin, "quasar10d07y265gmmuvt4z0w9aw880jnsr700j5pfcmk")

	// check that the market map was properly set
	mm, err := s.App.MarketmapKeeper.GetAllMarkets(s.Ctx)
	gotMM := marketmaptypes.MarketMap{Markets: mm}
	s.Require().NoError(err)
	s.Require().True(slinkyconstants.CoreMarketMap.Equal(gotMM))
	numCps, err := s.App.OracleKeeper.GetNumCurrencyPairs(s.Ctx)
	s.Require().NoError(err)
	s.Require().Equal(numCps, uint64(len(markets)))

	// check that all currency pairs have been initialized in the oracle module
	tickers := make([]slinkytypes.CurrencyPair, 0, len(markets))
	for _, market := range markets {
		decimals, err := s.App.OracleKeeper.GetDecimalsForCurrencyPair(s.Ctx, market.Ticker.CurrencyPair)
		s.Require().NoError(err)
		s.Require().Equal(market.Ticker.Decimals, decimals)

		price, err := s.App.OracleKeeper.GetPriceWithNonceForCurrencyPair(s.Ctx, market.Ticker.CurrencyPair)
		s.Require().NoError(err)
		s.Require().Equal(uint64(0), price.Nonce())     // no nonce because no updates yet
		s.Require().Equal(uint64(0), price.BlockHeight) // no block height because no price written yet

		s.Require().True(market.Ticker.Enabled)

		tickers = append(tickers, market.Ticker.CurrencyPair)
	}

	// check IDs for inserted currency pairs, sort currency-pairs alphabetically
	sort.Slice(tickers, func(i, j int) bool {
		return tickers[i].String() < tickers[j].String()
	})
	for i, cp := range tickers {
		id, found := s.App.OracleKeeper.GetIDForCurrencyPair(s.Ctx, cp)
		s.Require().True(found)
		s.Require().Equal(uint64(i), id)
	}
}

func (s *UpgradeTestSuite) TestEnableVoteExtensionsUpgrade() {
	s.Setup()
	s.PreBlockerSetup()

	app := s.App
	ctx := s.Ctx

	oldParams, err := app.ConsensusParamsKeeper.Params(ctx, &types.QueryParamsRequest{})
	s.Require().NoError(err)

	// VoteExtensionsEnableHeight must be updated after the upgrade on upgrade height
	// but the rest of params must be the same
	oldParams.Params.Abci = &comettypes.ABCIParams{VoteExtensionsEnableHeight: ctx.BlockHeight() + 4}
	// it is automatically tracked in upgrade handler, we need to set it manually for tests
	oldParams.Params.Version = &comettypes.VersionParams{App: 0}
	// we need to properly set consensus params for tests or we get a panic
	s.Require().NoError(app.ConsensusParamsKeeper.ParamsStore.Set(ctx, *oldParams.Params))

	upgrade := upgradetypes.Plan{
		Name:   v4.UpgradeName,
		Info:   "some text here",
		Height: ctx.BlockHeight(),
	}
	s.Require().NoError(app.UpgradeKeeper.ApplyUpgrade(ctx, upgrade))

	newParams, err := app.ConsensusParamsKeeper.Params(ctx, &types.QueryParamsRequest{})
	s.Require().NoError(err)

	s.Require().Equal(oldParams, newParams)
}

func dummyUpgrade(s *UpgradeTestSuite) {
	s.Ctx = s.Ctx.WithBlockHeight(v4UpgradeHeight - 1)
	plan := upgradetypes.Plan{Name: v4.Upgrade.UpgradeName, Height: v4UpgradeHeight}
	err := s.App.UpgradeKeeper.ScheduleUpgrade(s.Ctx, plan)
	s.Require().NoError(err)
	_, err = s.App.UpgradeKeeper.GetUpgradePlan(s.Ctx)
	s.Require().NoError(err)

	s.Ctx = s.Ctx.WithHeaderInfo(header.Info{Height: v4UpgradeHeight, Time: s.Ctx.BlockTime().Add(time.Second)}).WithBlockHeight(v4UpgradeHeight)
}

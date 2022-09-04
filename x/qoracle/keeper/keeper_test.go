package keeper_test

import (
	"testing"

	epochtypes "github.com/abag/quasarnode/osmosis/v9/epochs/types"
	minttypes "github.com/abag/quasarnode/osmosis/v9/mint/types"
	poolincentivestypes "github.com/abag/quasarnode/osmosis/v9/pool-incentives/types"
	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/suite"
)

type KeeperTestSuite struct {
	suite.Suite
	*testutil.TestSetup
}

func TestKeeperTestSuite(t *testing.T) {
	suite.Run(t, new(KeeperTestSuite))
}

func (suite *KeeperTestSuite) SetupTest() {
	suite.TestSetup = testutil.NewTestSetup(suite.T())
}

func (suite *KeeperTestSuite) SetStablePrices(prices sdk.DecCoins) {
	for _, p := range prices {
		suite.Keepers.QoracleKeeper.SetStablePrice(suite.Ctx, p.Denom, p.Amount)
	}
}

func (suite *KeeperTestSuite) SetOsmosisPools(pools []types.OsmosisPool) {
	for _, p := range pools {
		suite.Keepers.QoracleKeeper.SetOsmosisPool(suite.Ctx, p)
	}
}

func (suite *KeeperTestSuite) SetOsmosisParams(
	epochs []epochtypes.EpochInfo,
	distrInfo poolincentivestypes.DistrInfo,
	mintEpochProvisions sdk.Dec,
	mintParams minttypes.Params,
	incentivizedPools []poolincentivestypes.IncentivizedPool,
) {
	suite.Keepers.QoracleKeeper.SetOsmosisEpochsInfo(suite.Ctx, epochs)
	suite.Keepers.QoracleKeeper.SetOsmosisDistrInfo(suite.Ctx, distrInfo)
	suite.Keepers.QoracleKeeper.SetOsmosisMintEpochProvisions(suite.Ctx, mintEpochProvisions)
	suite.Keepers.QoracleKeeper.SetOsmosisMintParams(suite.Ctx, mintParams)
	suite.Keepers.QoracleKeeper.SetOsmosisIncentivizedPools(suite.Ctx, incentivizedPools)
}

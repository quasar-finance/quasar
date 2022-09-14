package keeper_test

import (
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	epochtypes "github.com/quasarlabs/quasarnode/osmosis/v9/epochs/types"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/v9/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/v9/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
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

func (suite *KeeperTestSuite) SetOraclePrices(prices sdk.DecCoins) {
	suite.Keepers.QoracleKeeper.SetOraclePrices(suite.Ctx, types.OraclePrices{
		Prices:          prices,
		UpdatedAtHeight: suite.Ctx.BlockHeight(),
	})
}

func (suite *KeeperTestSuite) SetDenomPriceMappings(m []types.DenomPriceMapping) {
	suite.Keepers.QoracleKeeper.SetDenomPriceMappings(suite.Ctx, m)
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

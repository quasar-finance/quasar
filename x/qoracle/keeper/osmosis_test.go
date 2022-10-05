package keeper_test

import (
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	minttypes "github.com/quasarlabs/quasarnode/osmosis/mint/types"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

const (
	TestOSMODenom  = "uosmo"
	TestATOMDenom  = "ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2"
	TestTerraDenom = "ibc/0EF15DF2F02480ADE0BB6E85D9EBB5DAEA2836D3860E9F97F9AADE4F57A31AA0"
	TestJunoDenom  = "ibc/46B44899322F3CD854D2D46DEEF881958467CDD4B3B10086DA49296BBED94BED"
	TestFakeDenom  = "ufake"
)

var (
	TestOraclePrices = sdk.NewDecCoins(
		sdk.NewInt64DecCoin("ATOM", 50000000),
		sdk.NewInt64DecCoin("OSMO", 15000000),
	)
	TestDenomPriceMappings = []types.DenomPriceMapping{
		{
			Denom:       TestATOMDenom,
			OracleDenom: "ATOM",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
		{
			Denom:       TestOSMODenom,
			OracleDenom: "OSMO",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
	}

	TestOsmosisPool1 = types.OsmosisPool{
		PoolInfo: balancerpool.Pool{
			Id: 1,
			PoolParams: balancerpool.PoolParams{
				SwapFee: sdk.ZeroDec(),
				ExitFee: sdk.ZeroDec(),
			},
			TotalShares: sdk.NewInt64Coin("share", 0),
			PoolAssets: []balancerpool.PoolAsset{
				{
					Token: sdk.NewInt64Coin(TestOSMODenom, 200),
				},
				{
					Token: sdk.NewInt64Coin(TestATOMDenom, 100),
				},
			},
			TotalWeight: sdk.ZeroInt(),
		},
		Metrics: types.OsmosisPoolMetrics{
			APY: sdk.MustNewDecFromStr("122.879"),
			TVL: sdk.MustNewDecFromStr("23456"),
		},
	}
	TestOsmosisPool2 = types.OsmosisPool{
		PoolInfo: balancerpool.Pool{
			Id: 2,
			PoolParams: balancerpool.PoolParams{
				SwapFee: sdk.ZeroDec(),
				ExitFee: sdk.ZeroDec(),
			},
			TotalShares: sdk.NewInt64Coin("share", 0),
			PoolAssets: []balancerpool.PoolAsset{
				{
					Token: sdk.NewInt64Coin(TestTerraDenom, 50),
				},
				{
					Token: sdk.NewInt64Coin(TestJunoDenom, 300),
				},
			},
			TotalWeight: sdk.ZeroInt(),
		},
		Metrics: types.OsmosisPoolMetrics{
			APY: sdk.MustNewDecFromStr("299.2"),
			TVL: sdk.MustNewDecFromStr("1568"),
		},
	}
	TestOsmosisPool3 = types.OsmosisPool{
		PoolInfo: balancerpool.Pool{
			Id: 3,
			PoolParams: balancerpool.PoolParams{
				SwapFee: sdk.ZeroDec(),
				ExitFee: sdk.ZeroDec(),
			},
			TotalShares: sdk.NewInt64Coin("share", 0),
			PoolAssets: []balancerpool.PoolAsset{
				{
					Token: sdk.NewInt64Coin(TestATOMDenom, 110),
				},
				{
					Token: sdk.NewInt64Coin(TestJunoDenom, 1000),
				},
			},
			TotalWeight: sdk.ZeroInt(),
		},
		Metrics: types.OsmosisPoolMetrics{
			APY: sdk.MustNewDecFromStr("105.69"),
			TVL: sdk.MustNewDecFromStr("11000"),
		},
	}
	TestOsmosisEpochs = []epochtypes.EpochInfo{
		{
			Identifier: "epoch-1",
			Duration:   time.Hour * 24 * 7,
		},
	}
	TestOsmosisDistrInfo = poolincentivestypes.DistrInfo{
		TotalWeight: sdk.NewInt(100),
		Records: []poolincentivestypes.DistrRecord{
			{
				GaugeId: 1,
				Weight:  sdk.NewInt(3),
			},
			{
				GaugeId: 2,
				Weight:  sdk.NewInt(4),
			},
		},
	}
	TestMintEpochProvisions = sdk.NewDec(1000)
	TestOsmosisMintParams   = minttypes.Params{
		EpochIdentifier: "epoch-1",
		MintDenom:       TestOSMODenom,
		DistributionProportions: minttypes.DistributionProportions{
			PoolIncentives: sdk.NewDec(4),
		},
	}
	TestOsmosisIncentivizedPools = []poolincentivestypes.IncentivizedPool{
		{
			PoolId:           1,
			LockableDuration: time.Hour * 24,
			GaugeId:          1,
		},
		{
			PoolId:           1,
			LockableDuration: time.Hour * 24 * 7,
			GaugeId:          2,
		},
	}
)

func (suite *KeeperTestSuite) TestCalculatePoolTVLByPoolId() {
	var pool balancerpool.Pool

	testCases := []struct {
		msg      string
		malleate func()
		expPass  bool
		expTVL   sdk.Dec
	}{
		{
			"success",
			func() {
				pool = balancerpool.Pool{
					Id: 1,
					PoolAssets: []balancerpool.PoolAsset{
						{
							Token:  sdk.NewInt64Coin(TestATOMDenom, 200),
							Weight: sdk.NewInt(2),
						},
						{
							Token:  sdk.NewInt64Coin(TestOSMODenom, 100),
							Weight: sdk.NewInt(1),
						},
					},
				}
			},
			true,
			sdk.NewDec(11500),
		},
		{
			"stable price not found",
			func() {
				pool = balancerpool.Pool{
					Id: 1,
					PoolAssets: []balancerpool.PoolAsset{
						{
							Token: sdk.NewInt64Coin(TestATOMDenom, 100),
						},
						{
							Token: sdk.NewInt64Coin(TestFakeDenom, 1000),
						},
					},
				}
			},
			false,
			sdk.ZeroDec(),
		},
	}

	for _, tc := range testCases {
		tc := tc

		suite.Run(tc.msg, func() {
			suite.SetupTest() // reset

			suite.SetOraclePrices(TestOraclePrices)
			suite.SetDenomPriceMappings(TestDenomPriceMappings)

			tc.malleate() // malleate mutates test data

			tvl, err := suite.Keepers.QoracleKeeper.CalculatePoolTVL(suite.Ctx, pool)

			if tc.expPass {
				suite.Require().NoError(err)
				suite.Require().Equal(tc.expTVL, tvl)
			} else {
				suite.Require().Error(err)
				suite.Require().Equal(sdk.ZeroDec(), tvl)
			}
		})
	}
}

func (suite *KeeperTestSuite) TestCalculatePoolAPYByPoolId() {
	var (
		pool balancerpool.Pool
		tvl  sdk.Dec
	)

	testCases := []struct {
		msg      string
		malleate func()
		expPass  bool
		expAPY   sdk.Dec
	}{
		{
			"success",
			func() {
				pool = balancerpool.Pool{
					Id: 1,
					PoolAssets: []balancerpool.PoolAsset{
						{
							Token:  sdk.NewInt64Coin(TestATOMDenom, 200),
							Weight: sdk.NewInt(2),
						},
						{
							Token:  sdk.NewInt64Coin(TestOSMODenom, 100),
							Weight: sdk.NewInt(1),
						},
					},
				}

				tvl = sdk.NewDec(11500)
			},
			true,
			sdk.MustNewDecFromStr("1899.130434782608695700"),
		},
	}

	for _, tc := range testCases {
		tc := tc

		suite.Run(tc.msg, func() {
			suite.SetupTest() // reset

			suite.SetOraclePrices(TestOraclePrices)
			suite.SetDenomPriceMappings(TestDenomPriceMappings)

			suite.SetOsmosisParams(
				TestOsmosisEpochs,
				TestOsmosisDistrInfo,
				TestMintEpochProvisions,
				TestOsmosisMintParams,
				TestOsmosisIncentivizedPools,
			)

			tc.malleate() // malleate mutates test data

			apy, err := suite.Keepers.QoracleKeeper.CalculatePoolAPY(suite.Ctx, pool, tvl)

			if tc.expPass {
				suite.Require().NoError(err)
				suite.Require().Equal(tc.expAPY, apy)
			} else {
				suite.Require().Error(err)
				suite.Require().Equal(sdk.ZeroDec(), apy)
			}
		})
	}
}

func (suite *KeeperTestSuite) TestGetOsmosisPoolsRankedByAPY() {
	var denom string

	testCases := []struct {
		msg      string
		malleate func()
		expPools []types.OsmosisPool
	}{
		{
			"success",
			func() {
				denom = ""
			},
			[]types.OsmosisPool{
				TestOsmosisPool2,
				TestOsmosisPool1,
				TestOsmosisPool3,
			},
		},
		{
			"success with denom filter",
			func() {
				denom = TestATOMDenom
			},
			[]types.OsmosisPool{
				TestOsmosisPool1,
				TestOsmosisPool3,
			},
		},
		{
			"empty result",
			func() {
				denom = TestFakeDenom
			},
			nil,
		},
	}

	for _, tc := range testCases {
		tc := tc

		suite.Run(tc.msg, func() {
			suite.SetupTest() // reset

			suite.SetOsmosisPools([]types.OsmosisPool{
				TestOsmosisPool3,
				TestOsmosisPool2,
				TestOsmosisPool1,
			})

			tc.malleate() // malleate mutates test data

			pools := suite.Keepers.QoracleKeeper.GetOsmosisPoolsRankedByAPY(suite.Ctx, denom)

			suite.Require().Equal(tc.expPools, pools)
		})
	}
}

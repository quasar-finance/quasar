package keeper_test

import (
	"time"

	epochtypes "github.com/abag/quasarnode/osmosis/v9/epochs/types"
	balancerpool "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	minttypes "github.com/abag/quasarnode/osmosis/v9/mint/types"
	poolincentivestypes "github.com/abag/quasarnode/osmosis/v9/pool-incentives/types"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

var (
	TestStablePrices = sdk.NewDecCoins(
		sdk.NewInt64DecCoin("ATOM", 50000000),
		sdk.NewInt64DecCoin("OSMO", 15000000),
	)

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
					Token: sdk.NewInt64Coin("uosmo", 200),
				},
				{
					Token: sdk.NewInt64Coin("uatom", 100),
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
					Token: sdk.NewInt64Coin("ukava", 50),
				},
				{
					Token: sdk.NewInt64Coin("ubnb", 300),
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
					Token: sdk.NewInt64Coin("uatom", 110),
				},
				{
					Token: sdk.NewInt64Coin("uband", 1000),
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
		MintDenom:       "uosmo",
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
							Token:  sdk.NewInt64Coin("uatom", 200),
							Weight: sdk.NewInt(2),
						},
						{
							Token:  sdk.NewInt64Coin("uosmo", 100),
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
							Token: sdk.NewInt64Coin("uatom", 100),
						},
						{
							Token: sdk.NewInt64Coin("ufake", 1000),
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

			suite.SetStablePrices(TestStablePrices)

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
							Token:  sdk.NewInt64Coin("uatom", 200),
							Weight: sdk.NewInt(2),
						},
						{
							Token:  sdk.NewInt64Coin("uosmo", 100),
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

			suite.SetStablePrices(TestStablePrices)
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

func (suite *KeeperTestSuite) TestGetOsmosisPoolsByDenom() {
	var denom string

	testCases := []struct {
		msg      string
		malleate func()
		expPools []types.OsmosisPool
	}{
		{
			"success",
			func() {
				denom = "uatom"
			},
			[]types.OsmosisPool{
				TestOsmosisPool1,
				TestOsmosisPool3,
			},
		},
		{
			"empty result",
			func() {
				denom = "ufake"
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

			pools := suite.Keepers.QoracleKeeper.GetOsmosisPoolsByDenom(suite.Ctx, denom)

			suite.Require().Equal(tc.expPools, pools)
		})
	}
}

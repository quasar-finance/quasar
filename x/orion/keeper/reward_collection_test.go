package keeper_test

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/nullify"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func createTestRewardCollection(keeper *keeper.Keeper, ctx sdk.Context) (types.RewardCollection, uint64) {
	item := types.RewardCollection{
		TimeCollected: time.Now().UTC(),
		Coins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
	}
	epochDay := uint64(42)
	keeper.SetRewardCollection(ctx, epochDay, item)
	return item, epochDay
}

func TestRewardCollection(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	item, epochDay := createTestRewardCollection(&k, ctx)
	rst, found := k.GetRewardCollection(ctx, epochDay)
	require.True(t, found)
	require.Equal(t,
		nullify.Fill(&item),
		nullify.Fill(&rst),
	)
}

func TestRemoveRewardCollection(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	_, epochDay := createTestRewardCollection(&k, ctx)
	k.RemoveRewardCollection(ctx, epochDay)
	_, found := k.GetRewardCollection(ctx, epochDay)
	require.False(t, found)
}

func TestDeductPerformanceFee(t *testing.T) {
	var tests = []struct {
		name        string
		profits     sdk.Coins
		expectError bool
		fee         sdk.Coins
	}{
		{
			name:        "no profit",
			profits:     sdk.NewCoins(),
			expectError: false,
			fee:         sdk.NewCoins(),
		},
		{
			name:        "single denom",
			profits:     sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(1000))),
			expectError: false,
			fee:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(30))),
		},
		{
			name: "two denoms",
			profits: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(1000)),
				sdk.NewCoin("def", sdk.NewInt(1500)),
			),
			expectError: false,
			fee: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(30)),
				sdk.NewCoin("def", sdk.NewInt(45)),
			),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			setup := testutil.NewTestSetup(t)
			ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
			mintErr := k.BankKeeper.MintCoins(ctx, types.CreateOrionRewardGloablMaccName(), tt.profits)
			if mintErr != nil {
				panic(mintErr)
			}
			oldRewardBalance := k.GetAllGlobalRewardBalances(ctx)
			oldPerfFeeBalance := k.GetFeeCollectorBalances(ctx, types.PerfFeeCollectorMaccName)
			fee, err := k.DeductPerformanceFee(ctx, tt.profits)
			newRewardBalance := k.GetAllGlobalRewardBalances(ctx)
			newPerfFeeBalance := k.GetFeeCollectorBalances(ctx, types.PerfFeeCollectorMaccName)
			if tt.expectError {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				require.True(t, fee.IsEqual(tt.fee))
				require.True(t, oldRewardBalance.Sub(fee).IsEqual(newRewardBalance))
				require.True(t, oldPerfFeeBalance.Add(fee...).IsEqual(newPerfFeeBalance))
			}
		})
	}
}

func TestCalculateDenomLPWeights(t *testing.T) {
	var tests = []struct {
		name        string
		totalLPV    sdk.Coins
		prices      map[string]sdk.Dec
		expectError bool
		weights     map[string]sdk.Dec
	}{
		{
			name:        "empty totalLPV",
			totalLPV:    sdk.NewCoins(),
			expectError: false,
			weights:     map[string]sdk.Dec{},
		},
		{
			name:        "single denom in totalLPV, no price data",
			totalLPV:    sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError: true,
		},
		{
			name:     "single denom in totalLPV, only orion price data",
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(2),
			},
			expectError: true,
		},
		{
			name:     "single denom in totalLPV, only denom price data",
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			prices: map[string]sdk.Dec{
				"abc": sdk.NewDec(2),
			},
			expectError: true,
		},
		{
			name:     "single denom in totalLPV, with both price data",
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(2),
				"abc":            sdk.NewDec(2),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDec(1),
			},
		},
		{
			name:     "single denom in totalLPV, arbitrary price data",
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDecWithPrec(123, 2),
				"abc":            sdk.NewDecWithPrec(687, 1),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDec(1),
			},
		},
		{
			name: "two denoms in totalLPV, same amount, same price",
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("def", sdk.NewInt(100)),
			),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(2),
				"abc":            sdk.NewDec(2),
				"def":            sdk.NewDec(2),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(5, 1),
				"def": sdk.NewDecWithPrec(5, 1),
			},
		},
		{
			name: "two denoms in totalLPV, same price",
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(80)),
				sdk.NewCoin("def", sdk.NewInt(120)),
			),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(2),
				"abc":            sdk.NewDec(2),
				"def":            sdk.NewDec(2),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(4, 1),
				"def": sdk.NewDecWithPrec(6, 1),
			},
		},
		{
			name: "two denoms in totalLPV, same amount",
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("def", sdk.NewInt(100)),
			),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(2),
				"abc":            sdk.NewDecWithPrec(35, 1),
				"def":            sdk.NewDecWithPrec(15, 1),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(7, 1),
				"def": sdk.NewDecWithPrec(3, 1),
			},
		},
		{
			name: "two denoms in totalLPV, both amount and price different, no truncation",
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(90)),
				sdk.NewCoin("def", sdk.NewInt(70)),
			),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(1),
				"abc":            sdk.NewDecWithPrec(35, 1),
				"def":            sdk.NewDecWithPrec(15, 1),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(75, 2),
				"def": sdk.NewDecWithPrec(25, 2),
			},
		},
		{
			name: "two denoms in totalLPV, both amount and price different",
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(150)),
				sdk.NewCoin("def", sdk.NewInt(100)),
			),
			prices: map[string]sdk.Dec{
				types.OrionDenom: sdk.NewDec(1),
				"abc":            sdk.NewDecWithPrec(35, 1),
				"def":            sdk.NewDecWithPrec(15, 1),
			},
			expectError: false,
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(777777777777777778, 18),
				"def": sdk.NewDecWithPrec(222222222222222222, 18),
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			setup := testutil.NewTestSetup(t)
			ctx, k, qoracleKeeper := setup.Ctx, setup.Keepers.OrionKeeper, setup.Keepers.QoracleKeeper
			for denom, price := range tt.prices {
				qoracleKeeper.SetStablePrice(ctx, denom, price)
			}
			weights, err := k.CalculateDenomLPWeights(ctx, tt.totalLPV)
			if tt.expectError {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				require.Equal(t, tt.weights, weights)
				totalWeight := sdk.ZeroDec()
				for _, w := range weights {
					require.True(t, w.GTE(sdk.ZeroDec()))
					totalWeight = totalWeight.Add(w)
				}
				require.True(t, totalWeight.LTE(sdk.NewDec(1)))
			}
		})
	}
}

func TestCalculateActualRewardForEachDenom(t *testing.T) {
	var tests = []struct {
		name         string
		netRewards   sdk.Coins
		weights      map[string]sdk.Dec
		expectError  bool
		denomRewards map[string]sdk.Coins
	}{
		{
			name:         "single weight (sum 1), no rewards",
			netRewards:   sdk.NewCoins(),
			weights:      map[string]sdk.Dec{"abc": sdk.NewDec(1)},
			expectError:  false,
			denomRewards: map[string]sdk.Coins{"abc": sdk.NewCoins()},
		},
		{
			name:         "single weight (sum 0.99), no rewards",
			netRewards:   sdk.NewCoins(),
			weights:      map[string]sdk.Dec{"abc": sdk.NewDecWithPrec(99, 2)},
			expectError:  false,
			denomRewards: map[string]sdk.Coins{"abc": sdk.NewCoins()},
		},
		{
			name:        "single weight (sum 1.01), no rewards",
			netRewards:  sdk.NewCoins(),
			weights:     map[string]sdk.Dec{"abc": sdk.NewDecWithPrec(101, 2)},
			expectError: true,
		},
		{
			name: "single weight, with rewards (one denom)",
			netRewards: sdk.NewCoins(
				sdk.NewCoin("osmo", sdk.NewInt(100)),
			),
			weights:     map[string]sdk.Dec{"abc": sdk.NewDec(1)},
			expectError: false,
			denomRewards: map[string]sdk.Coins{
				"abc": sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(100))),
			},
		},
		{
			name: "single weight, with rewards (two denoms)",
			netRewards: sdk.NewCoins(
				sdk.NewCoin("osmo", sdk.NewInt(100)),
				sdk.NewCoin("xyz1", sdk.NewInt(150)),
			),
			weights:     map[string]sdk.Dec{"abc": sdk.NewDec(1)},
			expectError: false,
			denomRewards: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(100)),
					sdk.NewCoin("xyz1", sdk.NewInt(150)),
				),
			},
		},
		{
			name:       "two weights, no rewards",
			netRewards: sdk.NewCoins(),
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(25, 2),
				"def": sdk.NewDecWithPrec(75, 2),
			},
			expectError: false,
			denomRewards: map[string]sdk.Coins{
				"abc": sdk.NewCoins(),
				"def": sdk.NewCoins(),
			},
		},
		{
			name:       "two weights, with rewards (one denom)",
			netRewards: sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(100))),
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(25, 2),
				"def": sdk.NewDecWithPrec(75, 2),
			},
			expectError: false,
			denomRewards: map[string]sdk.Coins{
				"abc": sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(25))),
				"def": sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(75))),
			},
		},
		{
			name: "two weights, with rewards (two denoms)",
			netRewards: sdk.NewCoins(
				sdk.NewCoin("osmo", sdk.NewInt(100)),
				sdk.NewCoin("xyz1", sdk.NewInt(150)),
			),
			weights: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(25, 2),
				"def": sdk.NewDecWithPrec(75, 2),
			},
			expectError: false,
			denomRewards: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(25)),
					sdk.NewCoin("xyz1", sdk.NewInt(37)),
				),
				"def": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(75)),
					sdk.NewCoin("xyz1", sdk.NewInt(112)),
				),
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			denomRewards, err := keeper.CalculateActualRewardForEachDenom(tt.netRewards, tt.weights)
			if tt.expectError {
				require.Error(t, err)
				return
			}
			require.NoError(t, err)
			require.Equal(t, len(tt.denomRewards), len(denomRewards))
			for denom, expectedReward := range tt.denomRewards {
				reward, exist := denomRewards[denom]
				require.True(t, exist)
				require.True(t, expectedReward.IsEqual(reward))
			}
			totalDenomRewards := sdk.NewCoins()
			for _, reward := range denomRewards {
				totalDenomRewards = totalDenomRewards.Add(reward...)
			}
			require.True(t, totalDenomRewards.IsAllLTE(tt.netRewards))
		})
	}
}

func TestCalculateUserRewards(t *testing.T) {
	var tests = []struct {
		name                  string
		activeUserDepositsMap map[string]sdk.Coins
		totalLPV              sdk.Coins
		denomActualReward     map[string]sdk.Coins
		userInfoMap           types.UserInfoMap
	}{
		{
			name:                  "all empty",
			activeUserDepositsMap: nil,
			totalLPV:              nil,
			denomActualReward:     nil,
			userInfoMap:           types.UserInfoMap{},
		},
		{
			name: "one active deposit, one deposited coin, no reward",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			},
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDec(1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(),
						},
					},
					TotalReward: sdk.NewCoins(),
				},
			},
		},
		{
			name: "one active deposit, one deposited coin, one reward coin",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			},
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(50))),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDec(1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(50))),
						},
					},
					TotalReward: sdk.NewCoins(sdk.NewCoin("osmo", sdk.NewInt(50))),
				},
			},
		},
		{
			name: "one active deposit, one deposited coin, multiple reward coin",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			},
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(50)),
					sdk.NewCoin("xyz1", sdk.NewInt(60)),
					sdk.NewCoin("xyz2", sdk.NewInt(70)),
				),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDec(1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(50)),
								sdk.NewCoin("xyz1", sdk.NewInt(60)),
								sdk.NewCoin("xyz2", sdk.NewInt(70)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(50)),
						sdk.NewCoin("xyz1", sdk.NewInt(60)),
						sdk.NewCoin("xyz2", sdk.NewInt(70)),
					),
				},
			},
		},
		{
			name: "one active deposit, two deposited coins, multiple reward coin",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(
					sdk.NewCoin("abc", sdk.NewInt(100)),
					sdk.NewCoin("def", sdk.NewInt(200)),
				),
			},
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("def", sdk.NewInt(200)),
			),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(50)),
					sdk.NewCoin("xyz1", sdk.NewInt(60)),
					sdk.NewCoin("xyz2", sdk.NewInt(70)),
				),
				"def": sdk.NewCoins(
					sdk.NewCoin("xyz2", sdk.NewInt(80)),
					sdk.NewCoin("xyz3", sdk.NewInt(90)),
				),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDec(1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(50)),
								sdk.NewCoin("xyz1", sdk.NewInt(60)),
								sdk.NewCoin("xyz2", sdk.NewInt(70)),
							),
						},
						"def": types.UserDenomInfo{
							Denom:  "def",
							Weight: sdk.NewDec(1),
							Amt:    sdk.NewInt(200),
							Reward: sdk.NewCoins(
								sdk.NewCoin("xyz2", sdk.NewInt(80)),
								sdk.NewCoin("xyz3", sdk.NewInt(90)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(50)),
						sdk.NewCoin("xyz1", sdk.NewInt(60)),
						sdk.NewCoin("xyz2", sdk.NewInt(150)),
						sdk.NewCoin("xyz3", sdk.NewInt(90)),
					),
				},
			},
		},
		{
			name: "two active deposits, one deposited coin, multiple reward coin",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
				"user2": sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			},
			totalLPV: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(200))),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(50)),
					sdk.NewCoin("xyz1", sdk.NewInt(60)),
					sdk.NewCoin("xyz2", sdk.NewInt(70)),
				),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDecWithPrec(5, 1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(25)),
								sdk.NewCoin("xyz1", sdk.NewInt(30)),
								sdk.NewCoin("xyz2", sdk.NewInt(35)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(25)),
						sdk.NewCoin("xyz1", sdk.NewInt(30)),
						sdk.NewCoin("xyz2", sdk.NewInt(35)),
					),
				},
				"user2": types.UserInfo{
					UserAcc: "user2",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDecWithPrec(5, 1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(25)),
								sdk.NewCoin("xyz1", sdk.NewInt(30)),
								sdk.NewCoin("xyz2", sdk.NewInt(35)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(25)),
						sdk.NewCoin("xyz1", sdk.NewInt(30)),
						sdk.NewCoin("xyz2", sdk.NewInt(35)),
					),
				},
			},
		},
		{
			name: "two active deposits, two deposited coins, multiple reward coin",
			activeUserDepositsMap: map[string]sdk.Coins{
				"user1": sdk.NewCoins(
					sdk.NewCoin("abc", sdk.NewInt(100)),
					sdk.NewCoin("def", sdk.NewInt(50)),
				),
				"user2": sdk.NewCoins(
					sdk.NewCoin("abc", sdk.NewInt(100)),
					sdk.NewCoin("def", sdk.NewInt(150)),
				),
			},
			totalLPV: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(200)),
				sdk.NewCoin("def", sdk.NewInt(200)),
			),
			denomActualReward: map[string]sdk.Coins{
				"abc": sdk.NewCoins(
					sdk.NewCoin("osmo", sdk.NewInt(50)),
					sdk.NewCoin("xyz1", sdk.NewInt(60)),
					sdk.NewCoin("xyz2", sdk.NewInt(70)),
				),
				"def": sdk.NewCoins(
					sdk.NewCoin("xyz2", sdk.NewInt(80)),
					sdk.NewCoin("xyz3", sdk.NewInt(90)),
				),
			},
			userInfoMap: types.UserInfoMap{
				"user1": types.UserInfo{
					UserAcc: "user1",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDecWithPrec(5, 1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(25)),
								sdk.NewCoin("xyz1", sdk.NewInt(30)),
								sdk.NewCoin("xyz2", sdk.NewInt(35)),
							),
						},
						"def": types.UserDenomInfo{
							Denom:  "def",
							Weight: sdk.NewDecWithPrec(25, 2),
							Amt:    sdk.NewInt(50),
							Reward: sdk.NewCoins(
								sdk.NewCoin("xyz2", sdk.NewInt(20)),
								sdk.NewCoin("xyz3", sdk.NewInt(22)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(25)),
						sdk.NewCoin("xyz1", sdk.NewInt(30)),
						sdk.NewCoin("xyz2", sdk.NewInt(55)),
						sdk.NewCoin("xyz3", sdk.NewInt(22)),
					),
				},
				"user2": types.UserInfo{
					UserAcc: "user2",
					DenomMap: map[string]types.UserDenomInfo{
						"abc": types.UserDenomInfo{
							Denom:  "abc",
							Weight: sdk.NewDecWithPrec(5, 1),
							Amt:    sdk.NewInt(100),
							Reward: sdk.NewCoins(
								sdk.NewCoin("osmo", sdk.NewInt(25)),
								sdk.NewCoin("xyz1", sdk.NewInt(30)),
								sdk.NewCoin("xyz2", sdk.NewInt(35)),
							),
						},
						"def": types.UserDenomInfo{
							Denom:  "def",
							Weight: sdk.NewDecWithPrec(75, 2),
							Amt:    sdk.NewInt(150),
							Reward: sdk.NewCoins(
								sdk.NewCoin("xyz2", sdk.NewInt(60)),
								sdk.NewCoin("xyz3", sdk.NewInt(67)),
							),
						},
					},
					TotalReward: sdk.NewCoins(
						sdk.NewCoin("osmo", sdk.NewInt(25)),
						sdk.NewCoin("xyz1", sdk.NewInt(30)),
						sdk.NewCoin("xyz2", sdk.NewInt(95)),
						sdk.NewCoin("xyz3", sdk.NewInt(67)),
					),
				},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			userInfoMap := keeper.CalculateUserRewards(tt.activeUserDepositsMap, tt.totalLPV, tt.denomActualReward)
			require.True(t, tt.userInfoMap.IsEqual(userInfoMap))
		})
	}
}

package keeper_test

import (
	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
	"testing"
)

func TestMintDeficit(t *testing.T) {
	var tests = []struct {
		name               string
		storedStablePrices map[string]sdk.Dec
		totalDeficit       sdk.Coins
		expectError        bool
		mintedOrions       map[string]sdk.Coin
		totalMinted        sdk.Coins
	}{
		{
			name:         "no deficit",
			totalDeficit: sdk.NewCoins(),
			expectError:  false,
			mintedOrions: map[string]sdk.Coin{},
			totalMinted:  sdk.NewCoins(),
		},
		{
			name:         "single coin, no price stored",
			totalDeficit: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError:  true,
		},
		{
			name: "single coin, different price stored",
			storedStablePrices: map[string]sdk.Dec{
				"xyz": sdk.NewDecWithPrec(12, 1),
			},
			totalDeficit: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError:  true,
		},
		{
			name: "single coin, no orion price",
			storedStablePrices: map[string]sdk.Dec{
				"abc": sdk.NewDecWithPrec(12, 1),
			},
			totalDeficit: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError:  true,
		},
		{
			name: "single coin, no qsr price",
			storedStablePrices: map[string]sdk.Dec{
				"abc":            sdk.NewDecWithPrec(12, 1),
				types.OrionDenom: sdk.NewDecWithPrec(9, 1),
			},
			totalDeficit: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError:  true,
		},
		{
			name: "single coin, valid",
			storedStablePrices: map[string]sdk.Dec{
				"abc":             sdk.NewDecWithPrec(12, 1),
				types.OrionDenom:  sdk.NewDecWithPrec(9, 1),
				types.QuasarDenom: sdk.NewDecWithPrec(22, 1),
			},
			totalDeficit: sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			expectError:  false,
			mintedOrions: map[string]sdk.Coin{
				"abc": sdk.NewCoin(types.OrionDenom, sdk.NewInt(133)),
			},
			totalMinted: sdk.NewCoins(sdk.NewCoin(types.OrionDenom, sdk.NewInt(133))),
		},
		{
			name: "two coins, valid",
			storedStablePrices: map[string]sdk.Dec{
				"abc":             sdk.NewDecWithPrec(12, 1),
				"def":             sdk.NewDecWithPrec(14, 1),
				types.OrionDenom:  sdk.NewDecWithPrec(9, 1),
				types.QuasarDenom: sdk.NewDecWithPrec(22, 1),
			},
			totalDeficit: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("def", sdk.NewInt(90)),
			),
			expectError: false,
			mintedOrions: map[string]sdk.Coin{
				"abc": sdk.NewCoin(types.OrionDenom, sdk.NewInt(133)),
				"def": sdk.NewCoin(types.OrionDenom, sdk.NewInt(140)),
			},
			totalMinted: sdk.NewCoins(sdk.NewCoin(types.OrionDenom, sdk.NewInt(273))),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			setup := testutil.NewTestSetup(t)
			ctx, k, qoracleKeeper := setup.Ctx, setup.Keepers.OrionKeeper, setup.Keepers.QoracleKeeper
			for denom, price := range tt.storedStablePrices {
				qoracleKeeper.SetStablePrice(ctx, denom, price)
			}
			mintedOrions, totalMinted, err := k.MintDeficit(ctx, tt.totalDeficit)
			if tt.expectError {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				require.EqualValues(t, tt.mintedOrions, mintedOrions)
				require.True(t, tt.totalMinted.IsEqual(totalMinted))
			}
		})
	}
}

func TestCalculateCoinAllocations(t *testing.T) {
	var tests = []struct {
		name            string
		neededCoins     sdk.Coins
		epochExitCoins  sdk.Coins
		reserveCoins    sdk.Coins
		fromEpochExit   sdk.Coins
		fromReserve     sdk.Coins
		excessEpochExit sdk.Coins
		totalDeficit    sdk.Coins
	}{
		{
			name:            "empty Coins",
			neededCoins:     sdk.NewCoins(),
			epochExitCoins:  sdk.NewCoins(),
			reserveCoins:    sdk.NewCoins(),
			fromEpochExit:   sdk.NewCoins(),
			fromReserve:     sdk.NewCoins(),
			excessEpochExit: sdk.NewCoins(),
			totalDeficit:    sdk.NewCoins(),
		},
		{
			name: "all from epoch exit",
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			epochExitCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			reserveCoins: sdk.NewCoins(),
			fromEpochExit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			fromReserve:     sdk.NewCoins(),
			excessEpochExit: sdk.NewCoins(),
			totalDeficit:    sdk.NewCoins(),
		},
		{
			name: "all from reserve",
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			epochExitCoins: sdk.NewCoins(),
			reserveCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			fromEpochExit: sdk.NewCoins(),
			fromReserve: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			excessEpochExit: sdk.NewCoins(),
			totalDeficit:    sdk.NewCoins(),
		},
		{
			name: "no epoch exit and no reserve",
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			epochExitCoins:  sdk.NewCoins(),
			reserveCoins:    sdk.NewCoins(),
			fromEpochExit:   sdk.NewCoins(),
			fromReserve:     sdk.NewCoins(),
			excessEpochExit: sdk.NewCoins(),
			totalDeficit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
		},
		{
			name: "with excess epoch coins",
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			epochExitCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(150)),
				sdk.NewCoin("abc2", sdk.NewInt(220)),
				sdk.NewCoin("abc3", sdk.NewInt(360)),
			),
			reserveCoins: sdk.NewCoins(),
			fromEpochExit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
			),
			fromReserve: sdk.NewCoins(),
			excessEpochExit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(50)),
				sdk.NewCoin("abc2", sdk.NewInt(20)),
				sdk.NewCoin("abc3", sdk.NewInt(60)),
			),
			totalDeficit: sdk.NewCoins(),
		},
		{
			name: "mixed",
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(200)),
				sdk.NewCoin("abc3", sdk.NewInt(300)),
				sdk.NewCoin("abc4", sdk.NewInt(80)),
			),
			epochExitCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(150)),
				sdk.NewCoin("abc2", sdk.NewInt(120)),
				sdk.NewCoin("abc3", sdk.NewInt(160)),
				sdk.NewCoin("xyz1", sdk.NewInt(40)),
			),
			reserveCoins: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(10)),
				sdk.NewCoin("abc2", sdk.NewInt(90)),
				sdk.NewCoin("abc3", sdk.NewInt(110)),
				sdk.NewCoin("xyz2", sdk.NewInt(70)),
			),
			fromEpochExit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(100)),
				sdk.NewCoin("abc2", sdk.NewInt(120)),
				sdk.NewCoin("abc3", sdk.NewInt(160)),
			),
			fromReserve: sdk.NewCoins(
				sdk.NewCoin("abc2", sdk.NewInt(80)),
				sdk.NewCoin("abc3", sdk.NewInt(110)),
			),
			excessEpochExit: sdk.NewCoins(
				sdk.NewCoin("abc1", sdk.NewInt(50)),
				sdk.NewCoin("xyz1", sdk.NewInt(40)),
			),
			totalDeficit: sdk.NewCoins(
				sdk.NewCoin("abc3", sdk.NewInt(30)),
				sdk.NewCoin("abc4", sdk.NewInt(80)),
			),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			fromEpochExit, fromReserve, excessEpochExit, totalDeficit := keeper.CalculateCoinAllocations(tt.neededCoins, tt.epochExitCoins, tt.reserveCoins)
			require.True(t, tt.fromEpochExit.IsEqual(fromEpochExit))
			require.True(t, tt.fromReserve.IsEqual(fromReserve))
			require.True(t, tt.excessEpochExit.IsEqual(excessEpochExit))
			require.True(t, tt.totalDeficit.IsEqual(totalDeficit))
		})
	}
}

func TestCalculateUserCoinsAndFees(t *testing.T) {
	var tests = []struct {
		name                     string
		depositedDenom           string
		depositorWeight          sdk.Dec
		availableCoins           sdk.Coins
		orionsMintedForEachDenom map[string]sdk.Coin
		mgmtFeePercentage        sdk.Dec
		userCoins                sdk.Coins
		mgmtFees                 sdk.Coins
	}{
		{
			name:                     "one available coin, no orion, no fees",
			depositedDenom:           "abc",
			depositorWeight:          sdk.NewDecWithPrec(1, 1),
			availableCoins:           sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(1000))),
			orionsMintedForEachDenom: make(map[string]sdk.Coin),
			mgmtFeePercentage:        sdk.ZeroDec(),
			userCoins:                sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			mgmtFees:                 sdk.NewCoins(),
		},
		{
			name:                     "one available coin, no orion, with fees",
			depositedDenom:           "abc",
			depositorWeight:          sdk.NewDecWithPrec(1, 1),
			availableCoins:           sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(1000))),
			orionsMintedForEachDenom: make(map[string]sdk.Coin),
			mgmtFeePercentage:        sdk.NewDecWithPrec(15, 2),
			userCoins:                sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(85))),
			mgmtFees:                 sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(15))),
		},
		{
			name:            "several available coin, no orion, with fees",
			depositedDenom:  "abc",
			depositorWeight: sdk.NewDecWithPrec(1, 1),
			availableCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(1000)),
				sdk.NewCoin("def", sdk.NewInt(500)),
			),
			orionsMintedForEachDenom: make(map[string]sdk.Coin),
			mgmtFeePercentage:        sdk.NewDecWithPrec(15, 2),
			userCoins:                sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(85))),
			mgmtFees:                 sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(15))),
		},
		{
			name:            "several available coin, no orion for deposited denom, with fees",
			depositedDenom:  "abc",
			depositorWeight: sdk.NewDecWithPrec(1, 1),
			availableCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(1000)),
				sdk.NewCoin("def", sdk.NewInt(500)),
			),
			orionsMintedForEachDenom: map[string]sdk.Coin{
				"def": sdk.NewCoin(types.OrionDenom, sdk.NewInt(1500)),
			},
			mgmtFeePercentage: sdk.NewDecWithPrec(15, 2),
			userCoins:         sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(85))),
			mgmtFees:          sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(15))),
		},
		{
			name:            "several available coin, with orion for deposited denom, with fees",
			depositedDenom:  "abc",
			depositorWeight: sdk.NewDecWithPrec(1, 1),
			availableCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(1000)),
				sdk.NewCoin("def", sdk.NewInt(500)),
			),
			orionsMintedForEachDenom: map[string]sdk.Coin{
				"abc": sdk.NewCoin(types.OrionDenom, sdk.NewInt(2000)),
				"def": sdk.NewCoin(types.OrionDenom, sdk.NewInt(1500)),
			},
			mgmtFeePercentage: sdk.NewDecWithPrec(15, 2),
			userCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(85)),
				sdk.NewCoin(types.OrionDenom, sdk.NewInt(170)),
			),
			mgmtFees: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(15)),
				sdk.NewCoin(types.OrionDenom, sdk.NewInt(30)),
			),
		},
		{
			name:            "truncation",
			depositedDenom:  "abc",
			depositorWeight: sdk.NewDecWithPrec(1, 2),
			availableCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(10050)),
				sdk.NewCoin("def", sdk.NewInt(5050)),
			),
			orionsMintedForEachDenom: map[string]sdk.Coin{
				"abc": sdk.NewCoin(types.OrionDenom, sdk.NewInt(20050)),
				"def": sdk.NewCoin(types.OrionDenom, sdk.NewInt(15050)),
			},
			mgmtFeePercentage: sdk.NewDecWithPrec(15, 2),
			userCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(85)),
				sdk.NewCoin(types.OrionDenom, sdk.NewInt(170)),
			),
			mgmtFees: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(15)),
				sdk.NewCoin(types.OrionDenom, sdk.NewInt(30)),
			),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			userCoins, mgmtFees := keeper.CalculateUserCoinsAndFees(tt.depositedDenom, tt.depositorWeight, tt.availableCoins, tt.orionsMintedForEachDenom, tt.mgmtFeePercentage)
			require.True(t, tt.userCoins.IsEqual(userCoins))
			require.True(t, tt.mgmtFees.IsEqual(mgmtFees))
		})
	}
}

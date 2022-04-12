package keeper

import (
	"github.com/stretchr/testify/require"
	"testing"

	gamm_types "github.com/abag/quasarnode/x/gamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func createSampleValidPoolAssetsSlice() []gamm_types.PoolAsset {
	return []gamm_types.PoolAsset{
		{
			Token:  sdk.NewCoin("abc", sdk.NewInt(80)),
			Weight: sdk.NewInt(3),
		},
		{
			Token:  sdk.NewCoin("xyz", sdk.NewInt(120)),
			Weight: sdk.NewInt(3),
		},
	}
}

func createSampleInvalidPoolAssetsSlice() []gamm_types.PoolAsset {
	return []gamm_types.PoolAsset{
		{
			Token:  sdk.NewCoin("abc", sdk.NewInt(80)),
			Weight: sdk.NewInt(3),
		},
		{
			Token:  sdk.NewCoin("xyz", sdk.ZeroInt()),
			Weight: sdk.NewInt(3),
		},
	}
}

func TestComputeShareOutAmount(t *testing.T) {
	var tests = []struct {
		name           string
		totalShares    sdk.Int
		poolAssets     []gamm_types.PoolAsset
		maxCoins       sdk.Coins
		error          bool
		shareOutAmount sdk.Int
	}{
		{
			name:  "empty pool assets",
			error: true,
		},
		{
			name:        "invalid pool assets",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleInvalidPoolAssetsSlice(),
			error:       true,
		},
		{
			name:           "no available coins",
			totalShares:    sdk.NewInt(100),
			poolAssets:     createSampleValidPoolAssetsSlice(),
			error:          false,
			shareOutAmount: sdk.ZeroInt(),
		},
		{
			name:           "one asset unavailable",
			totalShares:    sdk.NewInt(100),
			poolAssets:     createSampleValidPoolAssetsSlice(),
			maxCoins:       sdk.NewCoins(sdk.NewCoin("abc", sdk.NewInt(100))),
			error:          false,
			shareOutAmount: sdk.ZeroInt(),
		},
		{
			name:        "one coin incompatible",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleValidPoolAssetsSlice(),
			maxCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("eaf", sdk.NewInt(100)),
			),
			error:          false,
			shareOutAmount: sdk.ZeroInt(),
		},
		{
			name:        "one coin zero",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleValidPoolAssetsSlice(),
			maxCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("xyz", sdk.ZeroInt()),
			),
			error:          false,
			shareOutAmount: sdk.ZeroInt(),
		},
		{
			name:        "same ratio as pool assets",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleValidPoolAssetsSlice(),
			maxCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(120)),
				sdk.NewCoin("xyz", sdk.NewInt(180)),
			),
			error:          false,
			shareOutAmount: sdk.NewInt(150),
		},
		{
			name:        "first coin limiting",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleValidPoolAssetsSlice(),
			maxCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(60)),
				sdk.NewCoin("xyz", sdk.NewInt(120)),
			),
			error:          false,
			shareOutAmount: sdk.NewInt(75),
		},
		{
			name:        "second coin limiting",
			totalShares: sdk.NewInt(100),
			poolAssets:  createSampleValidPoolAssetsSlice(),
			maxCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(150)),
				sdk.NewCoin("xyz", sdk.NewInt(150)),
			),
			error:          false,
			shareOutAmount: sdk.NewInt(125),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			shareOutAmount, err := computeShareOutAmount(tt.totalShares, tt.poolAssets, tt.maxCoins)
			if tt.error {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				require.EqualValues(t, tt.shareOutAmount, shareOutAmount)
			}
		})
	}
}

func TestComputeNeededCoins(t *testing.T) {
	var tests = []struct {
		name              string
		totalSharesAmount sdk.Int
		shareOutAmount    sdk.Int
		poolAssets        []gamm_types.PoolAsset
		error             bool
		neededCoins       sdk.Coins
	}{
		{
			name:              "empty pool assets and zero total shares",
			totalSharesAmount: sdk.ZeroInt(),
			shareOutAmount:    sdk.NewInt(100),
			error:             false,
			neededCoins:       sdk.NewCoins(),
		},
		{
			name:              "non-empty pool assets and zero total shares",
			totalSharesAmount: sdk.ZeroInt(),
			shareOutAmount:    sdk.NewInt(100),
			poolAssets:        createSampleValidPoolAssetsSlice(),
			error:             true,
		},
		{
			name:              "want zero shares",
			totalSharesAmount: sdk.NewInt(100),
			shareOutAmount:    sdk.ZeroInt(),
			poolAssets:        createSampleValidPoolAssetsSlice(),
			error:             false,
			neededCoins:       sdk.NewCoins(),
		},
		{
			name:              "want half the total shares",
			totalSharesAmount: sdk.NewInt(100),
			shareOutAmount:    sdk.NewInt(50),
			poolAssets:        createSampleValidPoolAssetsSlice(),
			error:             false,
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(40)),
				sdk.NewCoin("xyz", sdk.NewInt(60)),
			),
		},
		{
			name:              "want 1.5 times the total shares",
			totalSharesAmount: sdk.NewInt(100),
			shareOutAmount:    sdk.NewInt(150),
			poolAssets:        createSampleValidPoolAssetsSlice(),
			error:             false,
			neededCoins: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(120)),
				sdk.NewCoin("xyz", sdk.NewInt(180)),
			),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			neededCoins, err := computeNeededCoins(tt.totalSharesAmount, tt.shareOutAmount, tt.poolAssets)
			if tt.error {
				require.Error(t, err)
			} else {
				require.NoError(t, err)
				// need to handle the zero case separately because sdk.Coins can have different
				// representations depending on how it's constructed.
				if tt.neededCoins.IsZero() {
					require.True(t, neededCoins.IsZero())
				} else {
					require.EqualValues(t, tt.neededCoins, neededCoins)
				}
			}
		})
	}
}

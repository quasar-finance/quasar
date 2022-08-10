package keeper_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/quasarlabs/quasarnode/testutil"
	gammbalancer "github.com/quasarlabs/quasarnode/x/intergamm/types/osmosis/v9/gamm/pool-models/balancer"
	orionkeeper "github.com/quasarlabs/quasarnode/x/orion/keeper"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbankmoduletypes "github.com/quasarlabs/quasarnode/x/qbank/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func createSampleValidPoolAssetsSlice() []gammbalancer.PoolAsset {
	return []gammbalancer.PoolAsset{
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

func createSampleInvalidPoolAssetsSlice() []gammbalancer.PoolAsset {
	return []gammbalancer.PoolAsset{
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

func stakeSampleTokens(k *orionkeeper.Keeper, ctx sdk.Context, lockupPeriod qbanktypes.LockupTypes, coins sdk.Coins) {
	if err := k.BankKeeper.MintCoins(ctx, types.OrionReserveMaccName, coins); err != nil {
		panic(err)
	}
	accName := types.CreateOrionStakingMaccName(lockupPeriod)
	if err := k.BankKeeper.SendCoinsFromModuleToModule(ctx, types.OrionReserveMaccName, accName, coins); err != nil {
		panic(err)
	}
}

func TestGetMaxAvailableTokensCorrespondingToPoolAssets(t *testing.T) {
	var tests = []struct {
		name              string
		lockupPeriod      qbanktypes.LockupTypes
		stakedCoins       sdk.Coins
		poolAssets        []gammbalancer.PoolAsset
		whiteListedDenoms []qbanktypes.WhiteListedDenomInOrion
		want              sdk.Coins
	}{
		{
			name:         "valid",
			lockupPeriod: qbankmoduletypes.LockupTypes_Days_7,
			stakedCoins: sdk.NewCoins(
				sdk.NewCoin("q-abc", sdk.NewInt(100)),
				sdk.NewCoin("q-def", sdk.NewInt(150)),
				sdk.NewCoin("q-xyz", sdk.NewInt(120)),
				sdk.NewCoin("q-zyx", sdk.NewInt(50)),
			),
			poolAssets: createSampleValidPoolAssetsSlice(),
			whiteListedDenoms: []qbanktypes.WhiteListedDenomInOrion{
				{
					OriginName:   "abc",
					OnehopQuasar: "q-abc",
					OnehopOsmo:   "abc",
				},
				{
					OriginName:   "xyz",
					OnehopQuasar: "q-xyz",
					OnehopOsmo:   "xyz",
				},
			},
			want: sdk.NewCoins(
				sdk.NewCoin("abc", sdk.NewInt(100)),
				sdk.NewCoin("xyz", sdk.NewInt(120)),
			),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			setup := testutil.NewTestSetup(t)
			ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
			qBankParams := setup.Keepers.QbankKeeper.GetParams(ctx)
			qBankParams.WhiteListedDenomsInOrion = tt.whiteListedDenoms
			setup.Keepers.QbankKeeper.SetParams(ctx, qBankParams)
			stakeSampleTokens(&k, ctx, tt.lockupPeriod, tt.stakedCoins)
			res := k.GetMaxAvailableTokensCorrespondingToPoolAssets(ctx, tt.lockupPeriod, tt.poolAssets)
			require.EqualValues(t, tt.want, res)
		})
	}
}

func TestComputeShareOutAmount(t *testing.T) {
	var tests = []struct {
		name           string
		totalShares    sdk.Int
		poolAssets     []gammbalancer.PoolAsset
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
			shareOutAmount, err := orionkeeper.ComputeShareOutAmount(tt.totalShares, tt.poolAssets, tt.maxCoins)
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
		poolAssets        []gammbalancer.PoolAsset
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
			neededCoins, err := orionkeeper.ComputeNeededCoins(tt.totalSharesAmount, tt.shareOutAmount, tt.poolAssets)
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

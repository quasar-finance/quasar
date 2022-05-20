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
			k.BankKeeper.MintCoins(ctx, types.CreateOrionRewardGloablMaccName(), tt.profits)
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

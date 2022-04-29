package qbank_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/qbank"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/stretchr/testify/require"
)

func TestGenesis(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QbankKeeper

	genesisState := types.GenesisState{
		Params: types.DefaultParams(),
		// this line is used by starport scaffolding # genesis/test/state
	}

	qbank.InitGenesis(ctx, k, genesisState)
	setParams := k.GetParams(ctx)
	require.Equal(t, genesisState.Params, setParams)
	got := qbank.ExportGenesis(ctx, k)
	require.NotNil(t, got)

	require.Equal(t, got.Params, (*got).Params)

	require.Equal(t, genesisState.Params, got.Params)
	require.ElementsMatch(t,
		genesisState.Params.WhiteListedDenomsInOrion,
		got.Params.WhiteListedDenomsInOrion)

	depositorAddr := sample.AccAddressStr()
	denom1 := "ABC"
	denom2 := "DEF"
	denom3 := "GHI"
	depositCoins := sdk.NewCoin(denom1, sdk.NewInt(50))
	totalDepositedCoin := sdk.NewCoin(denom1, sdk.NewInt(50))

	totalWithdrableCoin := sdk.NewCoin(denom1, sdk.NewInt(30))
	totalWithdrawsCoin := sdk.NewCoin(denom2, sdk.NewInt(10))

	claimableCoins := sdk.NewCoin(denom3, sdk.NewInt(100))
	claimedCoin := sdk.NewCoin(denom3, sdk.NewInt(100))

	currentEpoch := uint64(10)
	lockupDay := types.LockupTypes_Days_21
	k.AddUserDenomDeposit(ctx, depositorAddr, depositCoins)
	k.AddUserDeposit(ctx, depositorAddr, depositCoins)
	k.AddEpochLockupUserDenomDeposit(ctx,
		depositorAddr,
		depositCoins,
		currentEpoch,
		lockupDay)

	k.AddActualWithdrawableAmt(ctx, depositorAddr, totalWithdrableCoin)
	k.AddTotalWithdrawAmt(ctx, depositorAddr, "orion", sdk.NewCoins(totalWithdrawsCoin))

	k.AddUserClaimRewards(ctx, depositorAddr, "orion", sdk.NewCoins(claimableCoins))
	k.AddUserClaimedRewards(ctx, depositorAddr, "orion", sdk.NewCoins(claimedCoin))

	got = qbank.ExportGenesis(ctx, k)

	depositInfos := []types.DepositInfo{
		types.DepositInfo{
			VaultID:             "orion",
			EpochDay:            10,
			LockupPeriod:        lockupDay,
			DepositorAccAddress: depositorAddr,
			Coin:                depositCoins},
	}

	totalDeposits := []types.UserBalanceInfo{
		types.UserBalanceInfo{
			Type:                types.BalanceType_TOTAL_DEPOSIT,
			VaultID:             "orion",
			DepositorAccAddress: depositorAddr,
			Coins:               sdk.NewCoins(totalDepositedCoin)},
	}

	withdrawables := []types.UserBalanceInfo{
		types.UserBalanceInfo{
			Type:                types.BalanceType_WITHDRAWABLE,
			VaultID:             "orion",
			DepositorAccAddress: depositorAddr,
			Coins:               sdk.NewCoins(totalWithdrableCoin)},
	}

	totalWithdraws := []types.UserBalanceInfo{
		types.UserBalanceInfo{
			Type:                types.BalanceType_TOTAL_WITHDRAW,
			VaultID:             "orion",
			DepositorAccAddress: depositorAddr,
			Coins:               sdk.NewCoins(totalWithdrawsCoin)},
	}
	claimableRewards := []types.UserBalanceInfo{
		types.UserBalanceInfo{
			Type:                types.BalanceType_CLAIMABLE_REWARDS,
			VaultID:             "orion",
			DepositorAccAddress: depositorAddr,
			Coins:               sdk.NewCoins(claimableCoins)},
	}
	totalClaimedRewards := []types.UserBalanceInfo{
		types.UserBalanceInfo{
			Type:                types.BalanceType_TOTAL_CLAIMED_REWARDS,
			VaultID:             "orion",
			DepositorAccAddress: depositorAddr,
			Coins:               sdk.NewCoins(claimedCoin)},
	}

	expectedGenesis := types.GenesisState{
		Params:              types.DefaultParams(),
		DepositInfos:        depositInfos,
		TotalDeposits:       totalDeposits,
		Withdrawables:       withdrawables,
		TotalWithdraws:      totalWithdraws,
		ClaimableRewards:    claimableRewards,
		TotalClaimedRewards: totalClaimedRewards,
	}

	got = qbank.ExportGenesis(ctx, k)

	require.Equal(t, expectedGenesis, *got)
	require.ElementsMatch(t, depositInfos, got.DepositInfos)
	require.ElementsMatch(t, totalDeposits, got.TotalDeposits)
	require.ElementsMatch(t, withdrawables, got.Withdrawables)
	require.ElementsMatch(t, totalWithdraws, got.TotalWithdraws)
	require.ElementsMatch(t, claimableRewards, got.ClaimableRewards)
	require.ElementsMatch(t, totalClaimedRewards, got.TotalClaimedRewards)

	// this line is used by starport scaffolding # genesis/test/assert
}

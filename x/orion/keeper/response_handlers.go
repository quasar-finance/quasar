package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// OnJoinPoolAck handles the join pool acknowledgement
func (k Keeper) OnJoinPoolAck(ctx sdk.Context, packetSeq uint64, err error) {
	lp, lpErr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lpErr != nil {
		k.Logger(ctx).Info("OnJoinPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"internal_error", lpErr)
		return
	}

	if err != nil {
		lp.State = types.LpState_JOIN_FAILED
		k.Logger(ctx).Info("OnJoinPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"new lp State", lp.State)
		k.AddAvailableInterchainFund(ctx, lp.Coins)
		return
	}

	lp.State = types.LpState_JOINED
	k.setLpPosition(ctx, lp)

}

// OnJoinPoolTimeout handles the timeout condition for join pool requests
func (k Keeper) OnJoinPoolTimeout(ctx sdk.Context, packetSeq uint64) {
	lp, lpErr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lpErr != nil {
		k.Logger(ctx).Info("OnJoinPoolTimeout",
			"packetSeq", packetSeq,
			"internal_error", lpErr)
		return
	}
	lp.State = types.LpState_JOINING_TIMEOUT
	k.setLpPosition(ctx, lp)
	// Add the fund availability
	k.AddAvailableInterchainFund(ctx, lp.Coins)
}

func (k Keeper) OnExitPoolAck(ctx sdk.Context, packetSeq uint64, err error) error {
	lp, lpErr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lpErr != nil {
		k.Logger(ctx).Info("OnExitPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"internal_error", lpErr)
		return lpErr
	}

	if err != nil {
		lp.State = types.LpState_EXIT_FAILED
		k.Logger(ctx).Info("OnExitPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"new lp State", lp.State)
		return err
	}
	lp.State = types.LpState_EXITED
	k.setLpPosition(ctx, lp)
	tokensOut := k.computeTokenOutAmount(ctx, lp.Lptoken.Amount, lp.PoolID)
	for _, coin := range tokensOut {
		expectedExitDay := lp.BondingStartEpochDay + lp.BondDuration + lp.UnbondingDuration + 1
		k.AddEpochExitAmt(ctx, expectedExitDay, coin)
		err = k.TokenWithdrawFromOsmosis(ctx, coin)
		if err != nil {
			return err
		}
	}
	return nil
}

func (k Keeper) OnIBCTokenTransferAck(ctx sdk.Context, packetSeq uint64, err string) {
	ibcTokenTransfer, found := k.GetIBCTokenTransferRecord(ctx, packetSeq)
	if found {
		if len(err) == 0 {
			k.AddAvailableInterchainFund(ctx, sdk.NewCoins(ibcTokenTransfer.Coin))
		}
		k.DeleteIBCTokenTransferRecord(ctx, packetSeq)
	}
	k.Logger(ctx).Info("AfterEpochEnd", "available fund", k.GetAvailableInterchainFund(ctx))
}

func (k Keeper) OnIBCTokenTransferTimeout(ctx sdk.Context, packetSeq uint64) {

	// Deleting this values can cause issues regarding proper book keeping.
	// Orion need to either return that coins back to users or keep trying in some audit methods which
	// will get activated time to time and will clear previous days unprocessed data.
}

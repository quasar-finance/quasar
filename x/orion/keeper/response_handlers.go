package keeper

import (
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// OnJoinPoolAck handles the join pool acknowledgement
func (k Keeper) OnJoinPoolAck(ctx sdk.Context, packetSeq uint64, err error) {
	lp, lperr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lperr != nil {
		k.Logger(ctx).Info("OnJoinPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"internal_error", lperr)
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
	lp, lperr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lperr != nil {
		k.Logger(ctx).Info("OnJoinPoolTimeout",
			"packetSeq", packetSeq,
			"internal_error", lperr)
		return
	}
	lp.State = types.LpState_JOINING_TIMEOUT
	k.setLpPosition(ctx, lp)
	// Add the fund availability
	k.AddAvailableInterchainFund(ctx, lp.Coins)
}

func (k Keeper) OnExitPoolAck(ctx sdk.Context, packetSeq uint64, err error) {
	lp, lperr := k.GetLpPositionFromSeqNumber(ctx, packetSeq)
	if lperr != nil {
		k.Logger(ctx).Info("OnExitPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"internal_error", lperr)
		return
	}

	if err != nil {
		lp.State = types.LpState_EXIT_FAILED
		k.Logger(ctx).Info("OnExitPoolAck",
			"packetSeq", packetSeq,
			"error", err,
			"new lp State", lp.State)
	}
	lp.State = types.LpState_EXITED
	k.setLpPosition(ctx, lp)
	// Calculate expected - Exit funds based on current state of pool.
}

func (k Keeper) OnIBCTokenTransferAck(ctx sdk.Context, packetSeq uint64, ok bool) {
	if ok {
		coin, found := k.GetIBCTokenTransferRecord(ctx, packetSeq)
		if found {
			k.AddAvailableInterchainFund(ctx, sdk.NewCoins(coin))
			k.DeleteIBCTokenTransferRecord(ctx, packetSeq)
		}
	} else {

	}
}

func (k Keeper) OnIBCTokenTransferTimeout(ctx sdk.Context, packetSeq uint64) {
	// App level Retry max three times. Although retry is automatically done by the relayer.
	// Assuming that the ibc token transfer is robust and will return the requested amount.
	k.DeleteIBCTokenTransferRecord(ctx, packetSeq)
	// Should we return the amount back to user?
}

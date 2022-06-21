package keeper

import (
	"time"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
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

	// Lock LP Tokens
	duration := time.Duration(lp.UnbondingDuration * uint64(time.Hour))
	packetSeq, err = k.LockLPTokens(ctx, duration, sdk.NewCoins(lp.Lptoken))

	if err != nil {
		k.Logger(ctx).Info("OnJoinPoolAck - LockLPTokens Failed",
			"packetSeq", packetSeq,
			"error", err,
			"duration", duration,
			"lpToken", lp.Lptoken)
		return
	}

	k.SetSeqLockInfo(ctx, packetSeq, lp.LpID, lp.Lptoken, duration)
}

func (k Keeper) SetSeqLockInfo(ctx sdk.Context,
	packetSeq uint64,
	lpID uint64,
	lpShare sdk.Coin,
	duration time.Duration) {

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqLockTokensKBP)
	key := types.CreateSeqKey(packetSeq)
	lockInfo := types.LockInfo{LpID: lpID, Duration: duration, LpToken: lpShare}
	b := k.cdc.MustMarshal(&lockInfo)
	store.Set(key, b)
}

func (k Keeper) GetSeqLockInfo(ctx sdk.Context,
	packetSeq uint64) types.LockInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqLockTokensKBP)
	key := types.CreateSeqKey(packetSeq)
	bz := store.Get(key)
	var lockInfo types.LockInfo
	k.cdc.MustUnmarshal(bz, &lockInfo)
	return lockInfo
}

func (k Keeper) DeleteSeqLockInfo(ctx sdk.Context, packetSeq uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqLockTokensKBP)
	key := types.CreateSeqKey(packetSeq)
	store.Delete(key)
}

/////////

func (k Keeper) SetLockInfo(ctx sdk.Context, lockInfo types.LockInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LockTokensKBP)
	key := types.CreateLPIDKey(lockInfo.LpID)
	b := k.cdc.MustMarshal(&lockInfo)
	store.Set(key, b)
}

func (k Keeper) GetLockInfo(ctx sdk.Context,
	lpID uint64) types.LockInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LockTokensKBP)
	key := types.CreateLPIDKey(lpID)
	bz := store.Get(key)
	var lockInfo types.LockInfo
	k.cdc.MustUnmarshal(bz, &lockInfo)
	return lockInfo
}

func (k Keeper) DeleteLockInfo(ctx sdk.Context, lpID uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LockTokensKBP)
	key := types.CreateLPIDKey(lpID)
	store.Delete(key)
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
		seqNo, err := k.TokenWithdrawFromOsmosis(ctx, coin)
		if err != nil {
			return err
		}
		ibcWithdraw := types.IbcIcaWithdraw{SeqNo: seqNo, ExitEpochDay: expectedExitDay, Coin: coin}
		k.SetSeqTokenWithdrawFromOsmosis(ctx, ibcWithdraw)
	}
	return nil
}

func (k Keeper) OnTokenWithdrawFromOsmosis(ctx sdk.Context,
	packetSeq uint64, err string) {
	if len(err) != 0 {
		return
	}
	iw := k.GetSeqTokenWithdrawFromOsmosis(ctx, packetSeq)
	k.AddEpochExitAmt(ctx, iw.ExitEpochDay, iw.Coin) // packet forwarding cost to be adjusted during actual distribution
	k.DeleteSeqTokenWithdrawFromOsmosis(ctx, packetSeq)
}

func (k Keeper) OnIBCTokenTransferAck(ctx sdk.Context, packetSeq uint64, err string) {
	e, found1 := k.GetIBCTokenTransferRecord2(ctx, packetSeq)
	if found1 {
		if len(err) == 0 {
			k.AddAvailableInterchainFund(ctx, sdk.NewCoins(e.Coin))
		}

		k.SetTransferredEpochLockupCoins(ctx, e)
		k.DeleteIBCTokenTransferRecord2(ctx, packetSeq)
	}

	k.Logger(ctx).Info("AfterEpochEnd", "available fund", k.GetAvailableInterchainFund(ctx))
}

func (k Keeper) OnIBCTokenTransferTimeout(ctx sdk.Context, packetSeq uint64) {

	// Deleting this values can cause issues regarding proper book keeping.
	// Orion need to either return that coins back to users or keep trying in some audit methods which
	// will get activated time to time and will clear previous days unprocessed data.
}

func (k Keeper) OnLockTokenAck(ctx sdk.Context, packetSeq uint64, lockID uint64, err string) {
	if len(err) != 0 {
		return
	}

	lockInfo := k.GetSeqLockInfo(ctx, packetSeq)
	lockInfo.LockID = lockID
	k.SetLockInfo(ctx, lockInfo)
	k.DeleteSeqLockInfo(ctx, packetSeq)
}

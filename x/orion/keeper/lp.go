package keeper

import (
	"encoding/binary"
	"fmt"
	"strconv"
	"time"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
)

// NewLP create a new LPPosition object with input arguments.
// Zero Value of LpID, lockId means invalid values.
func (k Keeper) NewLP(lockId, bondingStartEpochDay, bondDuration, unbondingStartEpochDay,
	unbondingDuration, poolID uint64, state types.LpState, lpToken sdk.Coin, coins sdk.Coins) types.LpPosition {
	lp := types.LpPosition{LpID: 0,
		LockID:                 lockId,
		State:                  state,
		StartTime:              time.Now(),
		BondingStartEpochDay:   bondingStartEpochDay,
		BondDuration:           bondDuration,
		UnbondingStartEpochDay: unbondingStartEpochDay,
		UnbondingDuration:      unbondingDuration,
		PoolID:                 poolID,
		Lptoken:                lpToken,
		Coins:                  coins,
	}
	return lp
}

// GetDepositCount get the total number of deposit
func (k Keeper) GetLPCount(ctx sdk.Context) uint64 {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPCountKBP)
	byteKey := types.CreateLPCountKey()

	bz := store.Get(byteKey)

	// Count doesn't exist: no element
	if bz == nil {
		return 0
	}

	// Parse bytes
	return binary.BigEndian.Uint64(bz)
}

// SetDepositCount set the total number of deposit
func (k Keeper) setLPCount(ctx sdk.Context, count uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPCountKBP)
	byteKey := types.CreateLPCountKey()
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, count)
	store.Set(byteKey, bz)
}

// SetSeqNumber sets the mapping of seq number and lpID.
// Assumption - A fixed value of channel and port will be used.
func (k Keeper) SetSeqNumber(ctx sdk.Context, seqNumber uint64, lpId uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.JoinPoolKBP)
	byteKey := types.CreateSeqKey(seqNumber)
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, lpId)
	store.Set(byteKey, bz)
}

func (k Keeper) GetLpPositionFromSeqNumber(ctx sdk.Context, seqNumber uint64) (types.LpPosition, error) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.JoinPoolKBP)
	byteKey := types.CreateSeqKey(seqNumber)
	bz := store.Get(byteKey)
	if bz == nil {
		return types.LpPosition{}, fmt.Errorf("seq number %d not found", seqNumber)
	}
	lpID := binary.BigEndian.Uint64(bz)
	lp, ok := k.GetLpIdPosition(ctx, lpID)
	if !ok {
		return types.LpPosition{}, fmt.Errorf("lpID %d not found in kv store for seq number %d", lpID, seqNumber)
	}
	return lp, nil
}

// AddNewLPPosition update the LP ID of the newly created lp and set the position data in the KV store.
func (k Keeper) AddNewLPPosition(ctx sdk.Context, lpPosition types.LpPosition) uint64 {
	count := k.GetLPCount(ctx)
	lps, _ := k.GetLpStat(ctx, lpPosition.BondingStartEpochDay)
	lpPosition.LpID = count + 1   // Global count
	lps.LpCount = lps.LpCount + 1 // Epoch level count
	k.setLpPosition(ctx, lpPosition)
	k.setLpEpochPosition(ctx, lpPosition.LpID, lpPosition.BondingStartEpochDay)

	for _, coin := range lpPosition.Coins {
		lps.TotalLPCoins = lps.TotalLPCoins.Add(coin)
	}

	k.SetLpStat(ctx, lpPosition.BondingStartEpochDay, lps)

	// Helps to know the denom used in the particular epoch
	for _, coin := range lpPosition.Coins {
		k.SetEpochDenom(ctx, lpPosition.BondingStartEpochDay, coin.Denom)
	}
	k.setLPCount(ctx, lpPosition.LpID)
	return lpPosition.LpID
}

// SetLpPosition set lpPosition created by the strategy in a given epochDay in the
// prefixed kv store with key formed using epoch day and lpID.
// key = types.LPPositionKBP + {epochDay} + {":"} + {lpID}
// Value = types.LpPosition
func (k Keeper) setLpPosition(ctx sdk.Context, lpPosition types.LpPosition) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.EpochLPIDKey(lpPosition.BondingStartEpochDay, lpPosition.LpID)
	b := k.cdc.MustMarshal(&lpPosition)
	store.Set(key, b)
}

// SetLpEpochPosition set is used to store reverse mapping lpID and epochDay as part of key.
// Note - Ideally every entry in this should be an Active LP, Expired Lps should be removed from the system.
// key = types.LPEpochKBP + {lpID} + {":"} + {epochDay}
func (k Keeper) setLpEpochPosition(ctx sdk.Context, lpID uint64, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPEpochKBP)
	key := types.CreateLPEpochKey(lpID, epochDay)
	store.Set(key, []byte{0x00})
}

// SetEpochDenom set is used to store  mapping epochDay and denom as part of key.
// key = types.LPEpochDenomKBP + {epochDay} + {":"} + {denom}
func (k Keeper) SetEpochDenom(ctx sdk.Context, epochDay uint64, denom string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPEpochDenomKBP)
	key := types.CreateEpochDenomKey(epochDay, denom)
	store.Set(key, []byte{0x00})
}

// GetLPEpochDay fetch the epochDay of an lp position on which the position was created.
func (k Keeper) GetLPEpochDay(ctx sdk.Context, lpID uint64) (epochDay uint64, found bool) {
	prefixKey := types.CreateLPIDKey(lpID)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)

	// prefixKey => types.LPEpochKBP + {lpID} + {":"}
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPEpochKBP)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()
	// Note : Only one entry;  iteration will have maximum one loop
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		epochStr := string(key)
		epochDay, _ = strconv.ParseUint(epochStr, 10, 64)
		found = true
	}
	return epochDay, found
}

func (k Keeper) GetLpIdPosition(ctx sdk.Context, lpId uint64) (val types.LpPosition, found bool) {
	epochDay, found := k.GetLPEpochDay(ctx, lpId)
	if found {
		return k.GetLpPosition(ctx, epochDay, lpId)
	}
	return
}

// GetLpPosition fetch the lpPosition based on the epochDay and lpID input
func (k Keeper) GetLpPosition(ctx sdk.Context, epochDay uint64, lpID uint64) (val types.LpPosition, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.EpochLPIDKey(epochDay, lpID)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveLpPosition removes lpPosition from the store prefixed with epochDay and lpID
func (k Keeper) RemoveLpPosition(ctx sdk.Context, epochDay uint64, lpID uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.EpochLPIDKey(epochDay, lpID)
	store.Delete(key)
}

// GetLPIDList fetch the list of lp position lpId created on a given epoch day
func (k Keeper) GetLPIDList(ctx sdk.Context, epochDay uint64) []uint64 {
	var lpIDs []uint64
	prefixKey := types.EpochDayKey(epochDay)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)

	// prefixKey = types.LPPositionKBP + {epochDay} + {":"}
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		lpIDStr := string(key)
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)
		lpIDs = append(lpIDs, lpID)
	}
	return lpIDs
}

// GetActiveLpIDList get the list of currently active lpIDs on a given epochDay
// Ideally every entry present in this types.LPPositionKBP byte prefix corresponds
// to the active lpIDs, expired ones should be either removed or moved to the separate
// expired KV stores.
func (k Keeper) GetActiveLpIDList(ctx sdk.Context, epochDay uint64) []uint64 {

	var lpIDs []uint64
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	// key - {epochDay} + {":"} + {LPID}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		lpEpochDayStr := string(splits[0])
		lpEpochDay, _ := strconv.ParseUint(lpEpochDayStr, 10, 64)
		lpIDStr := string(splits[1])
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)

		// Cross-check for active
		lp, _ := k.GetLpPosition(ctx, lpEpochDay, lpID)
		lpEndDay := lp.BondingStartEpochDay + lp.BondDuration + lp.UnbondingDuration
		if lp.BondingStartEpochDay <= epochDay && epochDay <= lpEndDay {
			// Active LP
			lpIDs = append(lpIDs, lpID)
		}

	}
	return lpIDs
}

// GetAllLpIdList returns all the LpEpochPair present in the KV store.
func (k Keeper) GetAllLpEpochPairList(ctx sdk.Context) []types.LpEpochPair {
	var lpEpochs []types.LpEpochPair
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	// key - {epochDay} + {":"} + {LPID}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		epochDayStr := string(splits[0])
		epochDay, _ := strconv.ParseUint(epochDayStr, 10, 64)
		lpIDStr := string(splits[1])
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)
		lpEpochs = append(lpEpochs, types.LpEpochPair{LpId: lpID, EpochDay: epochDay})
	}
	return lpEpochs
}

// SetDayMapping is used to iterate and create tuple of target day( exit or reward), deposit day and lockup period.
// To further calculate the denom weights and users weights.
// Key = {DayMapKBP} +   {targetDay} + {":"} + {depositDay} + {":"} + {lockupPeriod}
// Target is happening every day - during thw whole periods.
// This map is also used for distribution of exited funds. target day/distribution day is same.
func (k Keeper) SetDayMapping(ctx sdk.Context, targetDay uint64,
	depositDay uint64, lockupPeriod qbanktypes.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DayMapKBP)
	key := types.CreateDayMappingKey(targetDay, depositDay, lockupPeriod)
	store.Set(key, []byte{0x00})
}

// GetDepositDayInfos gets the list of deposit day and lockup period for further processing.
// This method should be called every EOD with today epochDay = targetDay.
func (k Keeper) GetDepositDayInfos(ctx sdk.Context, targetDay uint64) []types.DepositDayLockupPair {
	prefixKey := types.EpochDayKey(targetDay)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DayMapKBP)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	var dls []types.DepositDayLockupPair

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetDepositDayInfos|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	// key = {depositDay} + {":"} + {lockupPeriod}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		bsplits := qbanktypes.SplitKeyBytes(key)
		depositDayStr := string(bsplits[0])
		depositDay, _ := strconv.ParseUint(depositDayStr, 10, 64)
		lockupPeriod := qbanktypes.LockupTypes_value[string(bsplits[1])]
		dl := types.DepositDayLockupPair{
			EpochDay:     depositDay,
			LockupPeriod: qbanktypes.LockupTypes(lockupPeriod)}
		dls = append(dls, dl)

	}

	return dls
}

///////////////////////////////////////////////////////////////////
//////////////////// LP STATS KV STORE METHODS ////////////////////
///////////////////////////////////////////////////////////////////

// SetLpStat set lpStat in the store with given epochDay and types.LpStat.
// This allows us to set the total ibc enabled sdk.Coin in an epoch
func (k Keeper) SetLpStat(ctx sdk.Context, epochDay uint64, lpStat types.LpStat) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochDay)
	b := k.cdc.MustMarshal(&lpStat)
	store.Set(byteKey, b)
}

// GetLpStat returns lpStat of a given epochDay
func (k Keeper) GetLpStat(ctx sdk.Context, epochDay uint64) (val types.LpStat, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochDay)
	b := store.Get(byteKey)
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveLpStat removes lpStat from the store of a given epochDay
func (k Keeper) RemoveLpStat(ctx sdk.Context, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochDay)
	store.Delete(byteKey)
}

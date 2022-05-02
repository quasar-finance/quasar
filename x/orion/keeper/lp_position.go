package keeper

import (
	"encoding/binary"
	"fmt"
	"strconv"
	"time"

	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// NewLP create a new LPPosition object with input arguments.
// Zero Value of LpID, lockid means invalid values.
func NewLP(lockid, bondingStartEpochday, bondDuration, unbondingStartEpochDay,
	unbondingDuration, poolID uint64, lpToken sdk.Coin, coins sdk.Coins) types.LpPosition {
	lp := types.LpPosition{LpID: 0,
		LockID:                 lockid,
		IsActive:               false,
		StartTime:              time.Now(),
		BondingStartEpochDay:   bondingStartEpochday,
		BondDuration:           bondDuration,
		UnbondingStartEpochDay: unbondingDuration,
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

// AddNewLPPosition update the LP ID of the newly created lp and set the position data in the KV store.
func (k Keeper) AddNewLPPosition(ctx sdk.Context, lpPosition types.LpPosition) {
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
}

// SetLpPosition set lpPosition created by the strategy in a given epochday in the
// prefixed kv store with key formed using epoch day and lpID.
// key = types.LPPositionKBP + {epochday} + {":"} + {lpID}
// Value = types.LpPosition
func (k Keeper) setLpPosition(ctx sdk.Context, lpPosition types.LpPosition) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPPositionKBP)
	key := types.EpochLPIDKey(lpPosition.BondingStartEpochDay, lpPosition.LpID)
	b := k.cdc.MustMarshal(&lpPosition)
	store.Set(key, b)
}

// SetLpEpochPosition set is used to store reverse mapping lpID and epochday as part of key.
// Note - Ideally every entry in this should be an Active LP, Expired Lps should be removed from the system.
// key = types.LPEpochKBP + {lpID} + {":"} + {epochDay}
func (k Keeper) setLpEpochPosition(ctx sdk.Context, lpID uint64, epochday uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPEpochKBP)
	key := types.CreateLPEpochKey(lpID, epochday)
	store.Set(key, []byte{0x00})
}

// SetEpochDenom set is used to store  mapping epochday and denom as part of key.
// key = types.LPEpochDenomKBP + {epochday} + {":"} + {denom}
func (k Keeper) SetEpochDenom(ctx sdk.Context, epochday uint64, denom string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPEpochDenomKBP)
	key := types.CreateEpochDenomKey(epochday, denom)
	store.Set(key, []byte{0x00})
}

// GetLPEpochDay fetch the epochday of an lp position on which the position was created.
func (k Keeper) GetLPEpochDay(ctx sdk.Context, lpID uint64) (epochday uint64, found bool) {
	bytePrefix := types.LPEpochKBP
	prefixKey := types.CreateLPIDKey(lpID)
	prefixKey = append(bytePrefix, prefixKey...)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)

	// prefixKey => types.LPEpochKBP + {lpID} + {":"}
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()
	// Note : Only one entry;  iteration will have maximum one loop
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		epochStr := string(key)
		epochday, _ = strconv.ParseUint(epochStr, 10, 64)
		found = true
	}
	return epochday, found
}

func (k Keeper) GetLpIdPosition(ctx sdk.Context, lpid uint64) (val types.LpPosition, found bool) {
	epochday, found := k.GetLPEpochDay(ctx, lpid)
	if found {
		return k.GetLpPosition(ctx, epochday, lpid)
	}
	return
}

// GetLpPosition fetch the lpPosition based on the epochday and lpID input
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

// GetLPIDList fetch the list of lp position lpid created on a given epoch day
func (k Keeper) GetLPIDList(ctx sdk.Context, epochday uint64) []uint64 {
	var lpIDs []uint64
	bytePrefix := types.LPPositionKBP
	prefixKey := types.EpochDayKey(epochday)
	prefixKey = append(bytePrefix, prefixKey...)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)

	// prefixKey = types.LPPositionKBP + {epochday} + {":"}
	store := ctx.KVStore(k.storeKey)
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

// GetActiveLpIDList get the list of currently active lpIDs on a given epochday
// Ideally every entry present in this types.LPPositionKBP byte prefix corresponds
// to the active lpID, all other expired should be either removed or moved to the separate
// expired KV stores.
func (k Keeper) GetActiveLpIDList(ctx sdk.Context, epochDay uint64) []uint64 {

	var lpIDs []uint64
	bytePrefix := types.LPPositionKBP
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, bytePrefix)
	defer iter.Close()

	// key - {epochday} + {":"} + {LPID}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		epochdayStr := string(splits[0])
		epochday, _ := strconv.ParseUint(epochdayStr, 10, 64)
		lpIDStr := string(splits[1])
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)

		// Cross check for active
		lp, _ := k.GetLpPosition(ctx, epochday, lpID)
		lpEndDay := lp.BondingStartEpochDay + lp.BondDuration + lp.UnbondingDuration
		if lp.BondingStartEpochDay <= epochday && epochday <= lpEndDay {
			// Active LP
			lpIDs = append(lpIDs, lpID)
		}

	}
	return lpIDs
}

// GetAllLpIdList returns all the LpEpochPair present in the KV store.
func (k Keeper) GetAllLpEpochPairList(ctx sdk.Context) []types.LpEpochPair {
	var lpepochs []types.LpEpochPair
	bytePrefix := types.LPPositionKBP
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, bytePrefix)
	defer iter.Close()

	// key - {epochday} + {":"} + {LPID}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		splits := qbanktypes.SplitKeyBytes(key)
		epochdayStr := string(splits[0])
		epochday, _ := strconv.ParseUint(epochdayStr, 10, 64)
		lpIDStr := string(splits[1])
		lpID, _ := strconv.ParseUint(lpIDStr, 10, 64)
		lpepochs = append(lpepochs, types.LpEpochPair{LpId: lpID, EpochDay: epochday})
	}
	return lpepochs
}

// GetDenomList fetch the list of denom used in an epoch day.
func (k Keeper) GetDenomList(ctx sdk.Context, epochday uint64) []string {
	var denoms []string
	bytePrefix := types.LPEpochDenomKBP
	prefixKey := types.EpochDayKey(epochday)
	prefixKey = append(bytePrefix, prefixKey...)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)

	// prefixKey = types.LPEpochDenomKBP + {epochday} + {":"}
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	// Key = denom
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		denom := string(key)
		denoms = append(denoms, denom)
	}
	return denoms
}

// AUDIT NOTE - Testing required
// CalculateLPWeight calc weight of an Lp position in the current epoch.
// This weight will be used for the approx fair reward distribution cross validation.
// Logic -
// 1. Get the lp position
// 2. Get current active gauge of lpID
// 3. Get expected apy
// 4. Get total  tvl*apy
// 5. Calc weight as ( < tvl of this lpid > * <apy of lpID> ) / Sum (tvl*apy)
// NOTE - Weight should be updated on each epoch in the object itself.
// There should be a dedicated field for this.
func (k Keeper) CalculateLPWeight(ctx sdk.Context, epochDay uint64, lpID uint64) (sdk.Dec, error) {
	lpp, found := k.GetLpPosition(ctx, epochDay, lpID)
	if !found {
		return sdk.ZeroDec(), fmt.Errorf("LP position not found")
	}
	k.Logger(ctx).Info(fmt.Sprintf("CalculateLPWeight|epochday=%v|lpp=%v\n", epochDay, lpp))
	g := k.GetCurrentActiveGauge(ctx, epochDay, lpID)
	tvl, _ := k.GetLpTvl(ctx, epochDay, lpID)
	tvlApy := g.ExpectedApy.Mul(tvl)
	totalTvlApy := k.GetTotalTvlApy(ctx, epochDay)
	lpw := tvlApy.Quo(totalTvlApy)
	return lpw, nil
}

// GetLpTvl calculate the total tvl in terms of amount of orion receipt tokens.
// Option #1 Normalize this in terms of one denom. For example - If it is pool#1 <atom, osmo>
// Then calculate this interms of atom or osmo
// Option #2 Calculate in terms of allocated orions amount.
func (k Keeper) GetLpTvl(ctx sdk.Context, epochday uint64, lpID uint64) (sdk.Dec, error) {
	lpp, found := k.GetLpPosition(ctx, epochday, lpID)
	if !found {
		return sdk.ZeroDec(), fmt.Errorf("LP position not found")
	}
	return lpp.Lptoken.Amount.ToDec(), nil
}

// GetTotalTvlApy calculate the total tvl in terms of amount of orion receipt tokens.
func (k Keeper) GetTotalTvlApy(ctx sdk.Context, epochDay uint64) sdk.Dec {
	lpi, _ := k.GetEpochLPInfo(ctx, epochDay)
	return lpi.TotalTVL.Amount.ToDec()
}

// GetCurrentActiveGauge fetch the currently active gauge of an LP position in the live chain
func (k Keeper) GetCurrentActiveGauge(ctx sdk.Context, epochday uint64, lpID uint64) types.GaugeLockInfo {
	var activeG types.GaugeLockInfo
	lp, _ := k.GetLpPosition(ctx, epochday, lpID)
	currEpochday := k.GetCurrentEpochDay(ctx)
	e, _ := k.GetEpochDayInfo(ctx, currEpochday)
	for _, g := range lp.Gaugelocks {
		if e.StartBlockTime.After(g.StartTime) && g.StartTime.Add(g.LockupDuration).After(e.EndBlockTime) {
			activeG = *g
		}
	}
	return activeG
}

// GetCurrentEpochDay is supposed to given current epochday
func (k Keeper) GetCurrentEpochDay(ctx sdk.Context) uint64 {
	// TO DO - Use the upcoming epoch module
	epochday := uint64(ctx.BlockHeader().Height)
	return epochday
}

// What is the denom weight contribution on a given epoch day?
// This will be used to calculate the users denom contribution which will be further used
// to calculate the users reward contribution for this denom

// GetEpochDenomWeight calculates the denom contribution to LPing on a given day.
// Logic -
// 1. Calculate Each denoms amount.
// 2. Get the total equivalent osmos or usdt for each denom.
// 3. Get total LP equivalent osmo/usdt/orions/share
// 4. Calculate denom weight based on its equivalent osmo/usdt/orions/share
func (k Keeper) GetEpochDenomWeight(ctx sdk.Context, epochday uint64) ([]types.EpochDenomWeight, error) {

	var edws []types.EpochDenomWeight
	lps, _ := k.GetLpStat(ctx, epochday)
	var totalOrionAmt sdk.Int
	denomOrionMap := make(map[string]sdk.Coin)
	for _, coin := range lps.TotalLPCoins {
		denomOrions, err := k.CalcReceipts(ctx, coin)
		if err != nil {
			// TODO recheck error handling
			return nil, err
		}
		totalOrionAmt = totalOrionAmt.Add(denomOrions.Amount)
		denomOrionMap[coin.Denom] = denomOrions
	}

	for _, coin := range lps.TotalLPCoins {
		denomOrion := denomOrionMap[coin.Denom]
		weight := denomOrion.Amount.ToDec().QuoInt(totalOrionAmt)
		dw := types.EpochDenomWeight{Denom: coin.Denom, Weight: weight}
		edws = append(edws, dw)
	}
	return edws, nil
}

// SetDayMapping is used to iterate and create tuple of reward day, deposit day and lockup period.
// To further calculate the denom weights and users weights.
// Key = {DayMapKBP} +   {rewardday} + {":"} + {depositday} + {":"} + {lockupPeriod}
// Reward is happening everyday - dueing thw whole periods.
// This map is also used for distribution of exited funds. reward day/distribution day is same.
func (k Keeper) SetDayMapping(ctx sdk.Context, rewardDay uint64,
	depositDay uint64, lockupPeriod qbanktypes.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DayMapKBP)
	key := types.CreateDayMappingKey(rewardDay, depositDay, lockupPeriod)
	store.Set(key, []byte{0x00})
}

// GetDepositDayInfos gets the list of deposit day and lockup period for further processing.
// This method should be called every EOD with today epochday = rewardday.
func (k Keeper) GetDepositDayInfos(ctx sdk.Context, rewardDay uint64) []types.DepositDayLockupPair {

	bytePrefix := types.DayMapKBP
	prefixKey := types.EpochDayKey(rewardDay)
	prefixKey = append(bytePrefix, prefixKey...)
	prefixKey = append(prefixKey, qbanktypes.SepByte...)
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	var dls []types.DepositDayLockupPair

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetDepositDayInfos|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	// key = {depositday} + {":"} + {lockupPeriod}
	for ; iter.Valid(); iter.Next() {
		key, _ := iter.Key(), iter.Value()
		bsplits := qbanktypes.SplitKeyBytes(key)
		depositdayStr := string(bsplits[0])
		depositday, _ := strconv.ParseUint(depositdayStr, 10, 64)
		lockupPeriod := qbanktypes.LockupTypes_value[string(bsplits[1])]
		dl := types.DepositDayLockupPair{
			Epochday:     depositday,
			LockupPeriod: qbanktypes.LockupTypes(lockupPeriod)}
		dls = append(dls, dl)

	}

	return dls
}

///////////////////////////////////////////////////////////////////
//////////////////// LP STATS KV STORE METHODS ////////////////////
///////////////////////////////////////////////////////////////////

// SetLpStat set lpStat in the store with given epochday and types.LpStat.
// This allows us to set the total ibc enabled sdk.Coin in an epoch
func (k Keeper) SetLpStat(ctx sdk.Context, epochday uint64, lpStat types.LpStat) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	b := k.cdc.MustMarshal(&lpStat)
	store.Set(byteKey, b)
}

// GetLpStat returns lpStat of a given epochday
func (k Keeper) GetLpStat(ctx sdk.Context, epochday uint64) (val types.LpStat, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	b := store.Get(byteKey)
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveLpStat removes lpStat from the store of a given epochday
func (k Keeper) RemoveLpStat(ctx sdk.Context, epochday uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPStatKBP)
	byteKey := types.EpochDayKey(epochday)
	store.Delete(byteKey)
}

package keeper

import (
	"fmt"

	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the current expected withdrawable amount /////////////////
///////////////////////////////////////////////////////////////////////////////////

// AUDIT NOTE - Discuss with team and decide. Possibly candidates for the removal. Because these analysis can be performed offline.
// GetWithdrawableAmt fetch the current withdrawable amount of a given user for a given denom
// from the KV store.
// Called for users expected withdrawable query and withdraw tx.
func (k Keeper) GetWithdrawableAmt(ctx sdk.Context, uid, denom string) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(denom, uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddWithdrawableAmt adds withdrawable amount from to the store for a given user and denom.
// Called from the qbank begin block
// Key = {denom} + ":" + {uid}
func (k Keeper) AddWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(coin.GetDenom(), uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		b := k.cdc.MustMarshal(&coin)
		store.Set(key, b)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

// SubWithdrawableAmt substracts withdrawable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
// AUDIT NOTE - This is tricky. Every time a user withdras - it should probably get empty.
// And users should be given full withdrawal option only.
// The expected withdrawable methods are anyway here for the purpose of possible analytics.
// Key = {denom} + ":" + {uid}
func (k Keeper) SubWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(coin.GetDenom(), uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrawable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		if !storedCoin.IsZero() {
			b = k.cdc.MustMarshal(&storedCoin)
			store.Set(key, b)
		} else {
			store.Delete(key)
		}
	}
}

// GetLockupWithdrawableAmt fetch the current withdrawable amount of a given user, denom and lockup period.
// from the lockperiod based KV store container. This method could be used for the analytics by external processes.
// Key = {denom} + ":" + {uid} + ":" + {lockupPeriod}
func (k Keeper) GetLockupWithdrawableAmt(ctx sdk.Context, uid, denom string, lockupPeriod types.LockupTypes) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrawableKey(denom, uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddLockupWithdrawableAmt adds withdrawable amount from to the store for a given user and denom.
// Called from the Orion vault end blocker.
func (k Keeper) AddLockupWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrawableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		b := k.cdc.MustMarshal(&coin)
		store.Set(key, b)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

// SubLockupWithdrawableAmt substracts withdrawable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
func (k Keeper) SubLockupWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrawableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrawable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the actual withdrawable amount ///////////////////////////
///////////////////////////////////////////////////////////////////////////////////

// GetActualWithdrawableAmt fetch the current withdrawable amount of a given user for a given denom
// from the KV store.
func (k Keeper) GetActualWithdrawableAmt(ctx sdk.Context, uid, denom string) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(uid, denom, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddActualWithdrawableAmt adds withdrawable amount from to the store for a given user and denom.
// Called from the Orion vault end blocker method DistributeEpochLockupFunds.
// Key = {uid} ":"   {denom}
func (k Keeper) AddActualWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(uid, coin.GetDenom(), types.Sep)
	b := store.Get(key)
	if b == nil {
		b := k.cdc.MustMarshal(&coin)
		store.Set(key, b)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

// SubActualWithdrawableAmt substracts withdrawable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
// Key = {denom} + ":" + {uid}
func (k Keeper) SubActualWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(uid, coin.GetDenom(), types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrawable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		if !storedCoin.IsZero() {
			b = k.cdc.MustMarshal(&storedCoin)
			store.Set(key, b)
		} else {
			store.Delete(key)
		}
	}
}

// EmptyActualWithdrawableAmt removes the key for uid and denom
func (k Keeper) EmptyActualWithdrawableAmt(ctx sdk.Context, uid, denom string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrawableKey(uid, denom, types.Sep)
	store.Delete(key)
}

// GetAllActualWithdrawables returns a list of all actual withdrawables for each depositor
func (k Keeper) GetAllActualWithdrawables(ctx sdk.Context) []types.UserBalanceInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetAllDepositInfos|modulename=%s|blockheight=%d",
		types.ModuleName, ctx.BlockHeight()))

	var userWithdrawablesMap = make(map[string]sdk.Coins)
	var totalWithdrawables []types.UserBalanceInfo

	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		splits := types.SplitKeyBytes(key)
		userAccStr := string(splits[0])
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		if _, found := userWithdrawablesMap[userAccStr]; found {
			userWithdrawablesMap[userAccStr] = userWithdrawablesMap[userAccStr].Add(coin)
		} else {
			userWithdrawablesMap[userAccStr] = sdk.NewCoins(coin)
		}
	}

	for userAccStr, coins := range userWithdrawablesMap {
		userWithdrawables := types.UserBalanceInfo{Type: types.BalanceType_WITHDRAWABLE,
			VaultID:             oriontypes.ModuleName,
			DepositorAccAddress: userAccStr,
			Coins:               coins,
		}
		totalWithdrawables = append(totalWithdrawables, userWithdrawables)
	}
	logger.Info("TotalWithdrawables", totalWithdrawables)
	return totalWithdrawables
}

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the users withdraw amount //////////////////////////////
///////////////////////////////////////////////////////////////////////////////////

// GetTotalWithdrawAmt fetch the total tokens withdraw so far from the previous deposits
// Key = types.TotalWithdrawKeyKBP + {uid} + ":" + {vaultID}
func (k Keeper) GetTotalWithdrawAmt(ctx sdk.Context, uid, vault string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.TotalWithdrawKeyKBP)
	key := types.CreateTotalWithdrawKey(uid, vault, types.Sep)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AddTotalWithdrawAmt adds total tokens withdraw from to the store for a given user and vault.
// Key = types.TotalWithdrawKeyKBP + {uid} + ":" + {vaultID}
func (k Keeper) AddTotalWithdrawAmt(ctx sdk.Context, uid, vaultID string, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.TotalWithdrawKeyKBP)
	key := types.CreateTotalWithdrawKey(uid, vaultID, types.Sep)
	b := store.Get(key)
	var qcoins types.QCoins
	if b == nil {
		qcoins.Coins = coins
		b := k.cdc.MustMarshal(&qcoins)
		store.Set(key, b)
	} else {
		var storedqcoins types.QCoins
		k.cdc.MustUnmarshal(b, &storedqcoins)
		storedqcoins.Coins = append(storedqcoins.Coins, coins...)
		value := k.cdc.MustMarshal(&storedqcoins)
		store.Set(key, value)
	}
}

// GetAllTotalWithdraws returns a list of all total withdraw tokens done so far for each users.
func (k Keeper) GetAllTotalWithdraws(ctx sdk.Context) []types.UserBalanceInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.TotalWithdrawKeyKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetAllWithdraw|modulename=%s|blockheight=%d",
		types.ModuleName, ctx.BlockHeight()))

	var totalWithdraws []types.UserBalanceInfo

	// key = {uid} + ":" + {vaultID}, value = types.QCoins
	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		splits := types.SplitKeyBytes(key)
		userAccStr := string(splits[0])
		var qcoin types.QCoins
		k.cdc.MustUnmarshal(value, &qcoin)

		userWithdraw := types.UserBalanceInfo{Type: types.BalanceType_TOTAL_WITHDRAW,
			VaultID:             oriontypes.ModuleName,
			DepositorAccAddress: userAccStr,
			Coins:               qcoin.Coins,
		}
		totalWithdraws = append(totalWithdraws, userWithdraw)
	}

	logger.Info("TotalWithdraws", totalWithdraws)
	return totalWithdraws
}

package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the current expected withdrable amount /////////////////
///////////////////////////////////////////////////////////////////////////////////

// AUDIT NOTE - Possibly candidates for the removal. Because these analysis can be performed offline.
// GetWithdrawableAmt fetch the current withdrable amount of a given user for a given denom
// from the KV store.
// Called for users expected withdrable query and withdraw tx.
func (k Keeper) GetWithdrawableAmt(ctx sdk.Context, uid, denom string) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrableKey(denom, uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddWithdrableAmt adds withdrable amount from to the store for a given user and denom.
// Called from the qbank begin block
// Key = {denom} + ":" + {uid}
func (k Keeper) AddWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrableKey(coin.GetDenom(), uid, types.Sep)
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

// SubWithdrableAmt substracts withdrable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
// AUDIT NOTE - This is tricky. Every time a user withdras - it should probably get empty.
// And users should be given full withdrawal option only.
// The expected withdrable methods are anyway here for the purpose of possible analytics.
// Key = {denom} + ":" + {uid}
func (k Keeper) SubWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateWithdrableKey(coin.GetDenom(), uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

// GetLockupWithdrawableAmt fetch the current withdrable amount of a given user, denom and lockup period.
// from the lockperiod based KV store container. This method could be used for the analytics by external processes.
// Key = {denom} + ":" + {uid} + ":" + {lockupPeriod}
func (k Keeper) GetLockupWithdrawableAmt(ctx sdk.Context, uid, denom string, lockupPeriod types.LockupTypes) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(denom, uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddWithdrableAmt adds withdrable amount from to the store for a given user and denom.
// Called from the Orion vault end blocker.
func (k Keeper) AddLockupWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
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

// SubWithdrableAmt substracts withdrable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
func (k Keeper) SubLockupWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.WithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the actual withdrable amount ///////////////////////////
///////////////////////////////////////////////////////////////////////////////////

// NOTE - Below  Add[XYZ] method for the actual withdraw amount will be called from the Orion module.
// GetActualWithdrawableAmt fetch the current withdrable amount of a given user for a given denom
// from the KV store.
// Called for users actual withdrable query and withdraw tx.
func (k Keeper) GetActualWithdrawableAmt(ctx sdk.Context, uid, denom string) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrableKey(denom, uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddActualWithdrableAmt adds withdrable amount from to the store for a given user and denom.
// Called from the Orion vault end blocker.
// Key = {denom} + ":" + {uid}
func (k Keeper) AddActualWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrableKey(coin.GetDenom(), uid, types.Sep)
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

// SubActualWithdrableAmt substracts withdrable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
// Key = {denom} + ":" + {uid}
func (k Keeper) SubActualWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateWithdrableKey(coin.GetDenom(), uid, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

// GetActualLockupWithdrawableAmt fetch the current withdrable amount of a given user, denom and lockup period.
// from the lockperiod based KV store container. This method could be used for the analytics by external processes.
// Key = {denom} + ":" + {uid} + ":" + {lockupPeriod}
func (k Keeper) GetActualLockupWithdrawableAmt(ctx sdk.Context, uid, denom string, lockupPeriod types.LockupTypes) (coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(denom, uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		return sdk.NewInt64Coin(denom, 0)
	}
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// AddActualLockupWithdrableAmt adds withdrable amount from to the store for a given user and denom.
// Called from the Orion vault end blocker.
func (k Keeper) AddActualLockupWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
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

// SubActualLockupWithdrableAmt substracts withdrable amount from to the store for a given user and denom.
// Called from the users withdraw transaction processing.
func (k Keeper) SubActualLockupWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
	key := types.CreateLockupWithdrableKey(coin.GetDenom(), uid, lockupPeriod, types.Sep)
	b := store.Get(key)
	if b == nil {
		// Do nothing. Ideally call should never come here.
		panic(fmt.Errorf("empty withdrable amount for key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		b = k.cdc.MustMarshal(&storedCoin)
		store.Set(key, b)
	}
}

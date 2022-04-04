package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetUserDepositAmount get the current value of user's total deposit amount.
// Key - types.UserDepositKBP + {useraccount}
func (k Keeper) GetUserDepositAmount(ctx sdk.Context, uid string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	b := store.Get(types.CreateUserDepositKey(uid))
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// AddUserDeposit adds user's deposit amount with key - types.UserDepositKBP + {uid} ,
// and will aggregate the total depsoited amount so far.
func (k Keeper) AddUserDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	key := types.CreateUserDepositKey(uid)
	b := store.Get(key)
	var qcoins types.QCoins
	if b == nil {
		qcoins.Coins = qcoins.Coins.Add(coin)
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		// Make sure that the stored coin set is in sorted order.
		// As the single coin element is always sorted, so the Add will never panic
		qcoins.Coins = qcoins.Coins.Add(coin)
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

// SubUserDeposit subs user's deposit amount with key - types.UserDepositKBP + {uid},
// and reduce the amount deposited so far.
func (k Keeper) SubUserDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	key := types.CreateUserDepositKey(uid)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake. Ideally code should never come here.
		panic(fmt.Sprintf("method SubUserDeposit |kv store does not have key=%v", string(key)))
	}

	var qcoins types.QCoins
	k.cdc.MustUnmarshal(b, &qcoins)
	// Make sure that the stored coin set is in sorted order.
	// As the single coin element is always sorted, so the Add will never panic
	qcoins.Coins = qcoins.Coins.Sub(sdk.NewCoins(coin))
	value := k.cdc.MustMarshal(&qcoins)
	store.Set(key, value)
}

// GetUserDenomDepositAmount get user's denom deposit amount.
// Key - types.UserDenomDepositKBP + {useraccount} + {":"} + {denom}
func (k Keeper) GetUserDenomDepositAmount(ctx sdk.Context, uid, denom string) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	b := store.Get(types.CreateUserDenomDepositKey(uid, types.Sep, denom))
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// AddUserDenomDeposit adds user's denom deposit amount with
// key - types.UserDenomDepositKBP + {uid} + {":"} + {denom}
func (k Keeper) AddUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, types.Sep, coin.GetDenom())
	b := store.Get(key)
	if b == nil {
		value := k.cdc.MustMarshal(&coin)
		store.Set(key, value)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// SubUserDenomDeposit subs user's denom deposit amount with
// key - types.UserDenomDepositKBP + {uid} + {":"} + {denom}
func (k Keeper) SubUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, types.Sep, coin.GetDenom())
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake. Ideally code should never come here.
		panic(fmt.Sprintf("method SubUserDenomDeposit |kv store does not have key=%v", string(key)))
	}

	var storedCoin sdk.Coin
	k.cdc.MustUnmarshal(b, &storedCoin)
	storedCoin = storedCoin.Sub(coin)
	value := k.cdc.MustMarshal(&storedCoin)
	store.Set(key, value)
}

// GetEpochLockupUserDenomDepositAmount get the current value of user's denom deposit amount
// with given epoch day and lockup period which is sdk.coin specifc to a given coin denom.
func (k Keeper) GetEpochLockupUserDenomDepositAmount(ctx sdk.Context,
	uid, denom string, epochday uint64, lockupPeriod types.LockupTypes) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, denom, epochday, lockupPeriod)

	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// AUDIT NOTE - This method could be redundant.
// GetUserDenomEpochLockupDepositAmount get user's denom deposit amount
// with given epoch day and lockup period
func (k Keeper) GetUserDenomEpochLockupDepositAmount(ctx sdk.Context,
	uid, denom string, epochday uint64, lockupPeriod types.LockupTypes) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomEpochLockupDepositKey(uid, types.Sep, denom, epochday, lockupPeriod)

	k.Logger(ctx).Info(fmt.Sprintf("GetUserDenomEpochLockupDepositAmount|key=%s|%s|%s|%v|%s\n",
		string(key), uid, denom, epochday, types.LockupTypes_name[int32(lockupPeriod)]))

	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// AddEpochLockupUserDenomDeposit adds user's denom deposit amount with
// Key - {UserDenomDepositKBP} +  ":" + {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func (k Keeper) AddEpochLockupUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, coin.GetDenom(), epochday, lockupPeriod)

	k.Logger(ctx).Info(fmt.Sprintf("AddEpochLockupUserDenomDeposit|key=%s|%s|%s|%s\n",
		string(key), uid, coin.Denom, types.LockupTypes_name[int32(lockupPeriod)]))

	b := store.Get(key)
	if b == nil {
		value := k.cdc.MustMarshal(&coin)
		store.Set(key, value)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// SubEpochLockupUserDenomDeposit subs user's denom deposit amount with
// Key - {UserDenomDepositKBP} +  ":" + {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func (k Keeper) SubEpochLockupUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, coin.GetDenom(), epochday, lockupPeriod)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake. Ideally code should never come here.
		panic(fmt.Sprintf("method SubEpochLockupUserDenomDeposit |kv store does not have key=%v", string(key)))
	}

	var storedCoin sdk.Coin
	k.cdc.MustUnmarshal(b, &storedCoin)
	storedCoin = storedCoin.Sub(coin)
	value := k.cdc.MustMarshal(&storedCoin)
	store.Set(key, value)
}

// GetTotalActiveDeposits calculates the current total active deposits.
// Logic -
// Iterate over { types.UserDepositKBP } => CreateUserDepositKey }
// Note - We need to guarantee actual deposit returns to the users irrespective of IL loss.
func (k Keeper) GetTotalActiveDeposits(ctx sdk.Context, module string) sdk.Coins {
	allCoins := sdk.NewCoins()
	bytePrefix := types.UserDepositKBP
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, bytePrefix)
	defer iter.Close()

	k.Logger(ctx).Info(fmt.Sprintf("GetTotalActiveDeposits|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(bytePrefix)))

	// key = userID, value = sdk.Coins
	for ; iter.Valid(); iter.Next() {
		_, value := iter.Key(), iter.Value()
		var qcoins types.QCoins
		k.cdc.MustUnmarshal(value, &qcoins)
		for _, c := range qcoins.Coins {
			allCoins = allCoins.Add(c)
		}
	}
	return allCoins
}

// GetEpochUserDepositAmt calculates the total deposit amount of the given users on a given day
// Iterate over all lockup periods, and prepare prefix key
// as - {epochday} + {:}+ {$lockupperiods} + {:} + {userAcc} + {:}
// On iteration, key - {denom}, value = sdk.Coin. No. of iteration can be upto the number of lockup periods
func (k Keeper) GetEpochUserDepositAmount(ctx sdk.Context, epochday uint64, userAcc string) sdk.Coins {
	bytePrefix := types.UserDenomDepositKBP
	var prefixKey []byte
	for lockupStr := range types.LockupTypes_value {
		prefixKey = types.CreateEpochLockupUserSepKey(epochday, lockupStr, userAcc, types.Sep)
	}

	prefixKey = append(bytePrefix, prefixKey...)
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	k.Logger(ctx).Info(fmt.Sprintf("GetEpochUserDepositAmount|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	var coins sdk.Coins
	for ; iter.Valid(); iter.Next() {
		// key = denom string byte, value = sdk.Coin marshled
		_, value := iter.Key(), iter.Value()
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		coins = coins.Add(coin)
		k.Logger(ctx).Info(fmt.Sprintf("GetEpochUserDepositAmount|modulename=%s|blockheight=%d|prefixKey=%s|coin=%v",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey), coin))

	}

	return coins
}

// GetEpochTotalActiveDeposits calculates the total amount deposited on a given epoch day
// Logic - Iterate with epoch day as prefix keys
// Full key -  {epochday} + {:}+ {$lockupperiods} + {:} + {userAcc} + {:} + {denom}
func (k Keeper) GetEpochTotalActiveDeposits(ctx sdk.Context, epochday uint64, moduleName string) sdk.Coins {
	bytePrefix := types.UserDenomDepositKBP
	prefixKey := types.EpochDaySepKey(epochday, types.Sep)
	prefixKey = append(bytePrefix, prefixKey...)
	store := ctx.KVStore(k.storeKey)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetEpochTotalActiveDeposits|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	var coins sdk.Coins
	for ; iter.Valid(); iter.Next() {
		// key = {$lockupperiods} + {:} + {userAcc} + {:} + {denom}, value = sdk.Coin marshled
		_, value := iter.Key(), iter.Value()
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		coins = coins.Add(coin)
		logger.Info(fmt.Sprintf("GetEpochTotalActiveDeposits|modulename=%s|blockheight=%d|prefixKey=%s|coin=%v",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey), coin))

	}

	return coins
}

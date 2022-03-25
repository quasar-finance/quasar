package keeper

import (
	"bytes"
	"encoding/binary"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// TODO - Add NewDeposit Method to easy the object construction.

// AUDIT NOTE - This method could be redundant.
// GetDepositCount get the total number of deposit
func (k Keeper) GetDepositCount(ctx sdk.Context) uint64 {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.QbankGlobalKBP)
	byteKey := types.CreateDepositCountKey()

	bz := store.Get(byteKey)

	// Count doesn't exist: no element
	if bz == nil {
		return 0
	}

	// Parse bytes
	return binary.BigEndian.Uint64(bz)
}

// AUDIT NOTE - This method could be redundant.
// SetDepositCount set the total number of deposit
func (k Keeper) SetDepositCount(ctx sdk.Context, count uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.QbankGlobalKBP)
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, count)
	store.Set(types.CreateDepositCountKey(), bz)
}

// AUDIT NOTE - This method could be redundant.
// AppendDeposit appends a deposit in the store with a new id and update the count
func (k Keeper) AppendDeposit(
	ctx sdk.Context,
	deposit types.Deposit,
) uint64 {
	// Create the deposit
	count := k.GetDepositCount(ctx)

	// Set the ID of the appended value
	deposit.Id = count

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)

	appendedValue := k.cdc.MustMarshal(&deposit)
	store.Set(types.CreateIDKey(deposit.Id), appendedValue)

	// Update deposit count
	k.SetDepositCount(ctx, count+1)

	return count
}

// AUDIT NOTE - This method could be redundant.
// SetDeposit set a specific deposit in the store. This should be used only in Init genesis.
func (k Keeper) SetDeposit(ctx sdk.Context, deposit types.Deposit) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	b := k.cdc.MustMarshal(&deposit)
	store.Set(types.CreateIDKey(deposit.Id), b)
}

// AUDIT NOTE - This method could be redundant.
// GetDeposit returns a deposit from its id
func (k Keeper) GetDeposit(ctx sdk.Context, id uint64) (val types.Deposit, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	b := store.Get(types.CreateIDKey(id))
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AUDIT NOTE - This method could be redundant.
// RemoveDeposit removes a deposit from the store
func (k Keeper) RemoveDeposit(ctx sdk.Context, id uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	store.Delete(types.CreateIDKey(id))
}

// AUDIT NOTE - This method could be redundant.
// GetAllDeposit returns all deposit.GetCoin().String()
// TODO TESTING | AUDIT
func (k Keeper) GetAllDeposit(ctx sdk.Context) (list []types.Deposit) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	iterator := sdk.KVStorePrefixIterator(store, []byte{}) // TODO TESTING
	defer iterator.Close()

	for ; iterator.Valid(); iterator.Next() {
		var val types.Deposit
		k.cdc.MustUnmarshal(iterator.Value(), &val)
		list = append(list, val)
	}

	return
}

// AUDIT NOTE - This method could be redundant.
// GetDepositIDBytes returns the byte representation of the ID
func GetDepositIDBytes(id uint64) []byte {
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, id)
	return bz
}

// AUDIT NOTE - This method could be redundant.
// GetDepositIDFromBytes returns ID in uint64 format from a byte array
func GetDepositIDFromBytes(bz []byte) uint64 {
	return binary.BigEndian.Uint64(bz)
}

// AUDIT NOTE - This method could be redundant.

// Get key used for user denom deposit in the KV store.
// TODO | AUDIT | Duplicate of types.CreateUserDenomDepositKey
func GetUserDenomDepositKey(uid, sep, denom string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	return b.Bytes()
}

// User Position Management

// AUDIT NOTE - This method could be redundant.
// Set user's denom deposit amount which is sdk.coin specifc to a given coin denom.
// Input denom examples - ATOM, OSMO, QSAR
// Not used
func (k Keeper) SetUserDenomDeposit(ctx sdk.Context, uid string, amount sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, types.Sep, amount.GetDenom())
	value := k.cdc.MustMarshal(&amount)
	store.Set(key, value)
}

// GetUserDenomDepositAmount get user's denom deposit amount which is sdk.coin specifc to a given coin denom.
func (k Keeper) GetUserDenomDepositAmount(ctx sdk.Context,
	uid, denom string) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	b := store.Get(types.CreateUserDenomDepositKey(uid, types.Sep, denom))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// GetUserDepositAmount get the current value of user's total deposit amount.
// Key - types.UserDepositKBP + user account.
func (k Keeper) GetUserDepositAmount(ctx sdk.Context, uid string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	b := store.Get(types.CreateUserDepositKey(uid))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// GetEpochLockupUserDenomDepositAmount get the current value of user's denom deposit amount
// with given epoch day and lockup period which is sdk.coin specifc to a given coin denom.
func (k Keeper) GetEpochLockupUserDenomDepositAmount(ctx sdk.Context,
	uid, denom string, epochday uint64, lockupPeriod types.LockupTypes) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	b := store.Get(types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, denom, epochday, lockupPeriod))
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

// AUDIT NOTE - This method could be redundant.
// GetUserDenomLockupDepositAmount get user's denom deposit amount with given lockup period
func (k Keeper) GetUserDenomLockupDepositAmount(ctx sdk.Context,
	uid, denom string, lockupPeriod types.LockupTypes) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomLockupDepositKey(uid, types.Sep, denom, lockupPeriod)
	// types.CreateUserDenomLockupDepositKey(uid, "/", coin.GetDenom(), lockupPeriod)
	// b := store.Get(types.CreateUserDenomLockupDepositKey(uid, "/", denom, lockupPeriod))

	k.Logger(ctx).Info(fmt.Sprintf("GetUserDenomLockupDepositAmount|key=%s|%s|%s|%s\n",
		string(key), uid, denom, types.LockupTypes_name[int32(lockupPeriod)]))

	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AUDIT NOTE - This method could be redundant.
// AddUserDenomEpochLockupDeposit adds user's deposit amount for the given epoch lockup period combination.
// Key - {UserDenomDepositKBP} + {uid} + ":" + {denom} + ":" + {epochday} + ":" + "lockupString"
func (k Keeper) AddUserDenomEpochLockupDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomEpochLockupDepositKey(uid, types.Sep, coin.GetDenom(), epochday, lockupPeriod)
	k.Logger(ctx).Info(fmt.Sprintf("AddUserDenomEpochLockupDeposit|key=%s|%s|%s|%d|%s\n",
		string(key), uid, coin.Denom, epochday, types.LockupTypes_name[int32(lockupPeriod)]))
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

// AUDIT NOTE - This method could be redundant.
// SubUserDenomEpochLockupDeposit substract user's denom deposit amount with
// Key - {UserDenomDepositKBP} + {uid} + ":" + {denom} + ":" + {epochday} + ":" + "lockupString"
func (k Keeper) SubUserDenomEpochLockupDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomEpochLockupDepositKey(uid, types.Sep, coin.GetDenom(), epochday, lockupPeriod)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		// TODO - panic.
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
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

// SubEpochLockupUserDenomDeposit subs user's denom deposit  amount with
// Key - {UserDenomDepositKBP} +  ":" + {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func (k Keeper) SubEpochLockupUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, coin.GetDenom(), epochday, lockupPeriod)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		// TODO - panic.
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// AddUserDenomLockupDeposit add user's denom deposit with
// key - {uid} + ":" + {denom} + ":" + {lockupString}
func (k Keeper) AddUserDenomLockupDeposit(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomLockupDepositKey(uid, types.Sep, coin.GetDenom(), lockupPeriod)

	k.Logger(ctx).Info(fmt.Sprintf("AddUserDenomLockupDeposit|key=%s|%s|%s|%s\n",
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

// SubUserDenomLockupDeposit Substract user's denom deposit amount with
// key - {UserDenomDepositKBP} + {uid} + ":" + {denom} + ":" + {lockupString}
func (k Keeper) SubUserDenomLockupDeposit(ctx sdk.Context, uid string, coin sdk.Coin, lockupPeriod types.LockupTypes) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomLockupDepositKey(uid, types.Sep, coin.GetDenom(), lockupPeriod)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		// TODO - panic.
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
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

// SubUserDenomDeposit subs user's denom deposit amount with
// key - types.UserDenomDepositKBP + {uid} + {":"} + {denom}
func (k Keeper) SubUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, types.Sep, coin.GetDenom())
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		// TODO - panic.
		//value := k.cdc.MustMarshal(&coin)
		//store.Set(key, value)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
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
		// Do nothing - Called by mistake.
		// TODO - panic.

	} else {
		var qcoins types.QCoins
		k.cdc.MustUnmarshal(b, &qcoins)
		// Make sure that the stored coin set is in sorted order.
		// As the single coin element is always sorted, so the Add will never panic
		qcoins.Coins = qcoins.Coins.Sub(sdk.NewCoins(coin))
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

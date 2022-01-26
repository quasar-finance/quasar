package keeper

import (
	"bytes"
	"encoding/binary"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// TODO - Add NewDeposit Method to easy the object construction.

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

// SetDepositCount set the total number of deposit
func (k Keeper) SetDepositCount(ctx sdk.Context, count uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.QbankGlobalKBP)
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, count)
	store.Set(types.CreateDepositCountKey(), bz)
}

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
	store.Set(GetDepositIDBytes(deposit.Id), appendedValue)

	// Update deposit count
	k.SetDepositCount(ctx, count+1)

	return count
}

// SetDeposit set a specific deposit in the store
func (k Keeper) SetDeposit(ctx sdk.Context, deposit types.Deposit) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	b := k.cdc.MustMarshal(&deposit)
	store.Set(GetDepositIDBytes(deposit.Id), b)
}

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

// RemoveDeposit removes a deposit from the store
func (k Keeper) RemoveDeposit(ctx sdk.Context, id uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.DepositKBP)
	store.Delete(types.CreateIDKey(id))
}

// GetAllDeposit returns all deposit
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

// GetDepositIDBytes returns the byte representation of the ID
func GetDepositIDBytes(id uint64) []byte {
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, id)
	return bz
}

// GetDepositIDFromBytes returns ID in uint64 format from a byte array
func GetDepositIDFromBytes(bz []byte) uint64 {
	return binary.BigEndian.Uint64(bz)
}

// Get key used for user denom deposit in the KV store.
func GetUserDenomDepositKey(uid, sep, denom string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	return b.Bytes()
}

// User Position Management

// Set user's denom deposit amount which is sdk.coin specifc to a given coin denom.
// Input denom examples - ATOM, OSMO, QSAR
func (k Keeper) SetUserDenomDeposit(ctx sdk.Context, uid string, amount sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, "/", amount.GetDenom())
	value := k.cdc.MustMarshal(&amount)
	store.Set(key, value)
}

// Get user's denom deposit amount which is sdk.coin specifc to a given coin denom.
func (k Keeper) GetUserDenomDepositAmount(ctx sdk.Context,
	uid, denom string) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	b := store.Get(types.CreateUserDenomDepositKey(uid, "/", denom))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// Get user's deposit amount in qbank module types.QCoins
func (k Keeper) GetUserDepositAmount(ctx sdk.Context, uid string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	b := store.Get(types.CreateUserDepositKey(uid))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

/*
// Add user's lockup period wise denom deposit amount which is sdk.coin specifc to a given coin denom.
// Input denom examples - ATOM, OSMO, QSAR
func (k Keeper) AddUserDenomLockupDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, "/", coin.GetDenom())
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
*/

// Add user's denom deposit amount which is sdk.coin specifc to a given coin denom.
// Input denom examples - ATOM, OSMO, QSAR
func (k Keeper) AddUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, "/", coin.GetDenom())
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

// Add users deposit amount which is sdk.coin specific to a given denom into the
// total depsopit amounts so far as qbank types.QCoins value)
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

// Substract user's denom deposit amount which is sdk.coin specifc to a given coin denom.
// Input denom examples - ATOM, OSMO, QSAR
func (k Keeper) SubUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	key := types.CreateUserDenomDepositKey(uid, "/", coin.GetDenom())
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

// substract users deposit amount which is sdk.coin specific to a given denom from the
// total depsopit amounts so far as qbank types.QCoins value)
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

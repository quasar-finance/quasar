package keeper

import (
	"encoding/binary"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetDepositCount get the total number of deposit
func (k Keeper) GetDepositCount(ctx sdk.Context) uint64 {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), []byte{})
	byteKey := types.KeyPrefix(types.DepositCountKey)
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
	store := prefix.NewStore(ctx.KVStore(k.storeKey), []byte{})
	byteKey := types.KeyPrefix(types.DepositCountKey)
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, count)
	store.Set(byteKey, bz)
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

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.DepositKey))
	appendedValue := k.cdc.MustMarshal(&deposit)
	store.Set(GetDepositIDBytes(deposit.Id), appendedValue)

	// Update deposit count
	k.SetDepositCount(ctx, count+1)

	return count
}

// SetDeposit set a specific deposit in the store
func (k Keeper) SetDeposit(ctx sdk.Context, deposit types.Deposit) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.DepositKey))
	b := k.cdc.MustMarshal(&deposit)
	store.Set(GetDepositIDBytes(deposit.Id), b)
}

// GetDeposit returns a deposit from its id
func (k Keeper) GetDeposit(ctx sdk.Context, id uint64) (val types.Deposit, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.DepositKey))
	b := store.Get(GetDepositIDBytes(id))
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveDeposit removes a deposit from the store
func (k Keeper) RemoveDeposit(ctx sdk.Context, id uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.DepositKey))
	store.Delete(GetDepositIDBytes(id))
}

// GetAllDeposit returns all deposit
func (k Keeper) GetAllDeposit(ctx sdk.Context) (list []types.Deposit) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.DepositKey))
	iterator := sdk.KVStorePrefixIterator(store, []byte{})

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

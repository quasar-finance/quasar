package keeper

import (
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetFeeData set feeData in the store
func (k Keeper) SetFeeData(ctx sdk.Context, feeData types.FeeData) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FeeDataKey))
	b := k.cdc.MustMarshal(&feeData)
	store.Set([]byte{0}, b)
}

// GetFeeData returns feeData
func (k Keeper) GetFeeData(ctx sdk.Context) (val types.FeeData, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FeeDataKey))

	b := store.Get([]byte{0})
	if b == nil {
		return val, false
	}

	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveFeeData removes feeData from the store
func (k Keeper) RemoveFeeData(ctx sdk.Context) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyPrefix(types.FeeDataKey))
	store.Delete([]byte{0})
}

package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetUserLPInfo set userLPInfo in the store
func (k Keeper) SetUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string, userLPInfo types.UserLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	value := k.cdc.MustMarshal(&userLPInfo)
	store.Set(key, value)
}

// GetUserLPInfo returns userLPInfo
func (k Keeper) GetUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string) (val types.UserLPInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// RemoveUserLPInfo removes userLPInfo from the store
func (k Keeper) RemoveUserLPInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	store.Delete(key)
}

// AddEpochLPUser add kv store with key = {epochday} + {":"} + {lpID} + {":"} + {userAccount}
// value = UserLPInfo. This method is to be used for once time only
func (k Keeper) AddEpochLPUserInfo(ctx sdk.Context, epochday uint64, lpID uint64, userAcc string, userLPInfo types.UserLPInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.LPUserInfoKBP)
	key := types.CreateEpochLPUserInfo(epochday, lpID, userAcc)
	value := k.cdc.MustMarshal(&userLPInfo)
	store.Set(key, value)

}

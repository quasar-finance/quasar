package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetUserClaimAmount get the current value of user's total claimable amount.
// Key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) GetUserClaimAmount(ctx sdk.Context, uid, vaultID string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	b := store.Get(types.CreateUsersClaimKey(uid, vaultID, types.Sep))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AddUserClaimReward adds user's claim amount. This method is called by orion vault
// key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) AddUserClaimReward(ctx sdk.Context, uid, vaultID string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
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

// AddUserClaimRewards adds user's claim amount in sdk.Coins. This method is called by orion vault
// key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) AddUserClaimRewards(ctx sdk.Context, uid, vaultID string, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
	b := store.Get(key)
	var qcoins types.QCoins
	if b == nil {
		qcoins.Coins = coins // AUDIT the slice usage here.
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		// Make sure that the stored coin set is in sorted order.
		// As the single coin element is always sorted, so the Add will never panic
		for _, coin := range coins {
			qcoins.Coins = qcoins.Coins.Add(coin)
		}
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

// Claim will remove users key from the KV store value for the requested user
func (k Keeper) ClaimAll(ctx sdk.Context, uid, vaultID string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
	store.Delete(key)
}

// NOTE - Not used now.
// A possible use case for the future for this method is when users want to use a part of his
// reward for other purpose - like re-investing in the treasury; once those features are supported.
// SubUserClaimReward subs user's claim amount. This method is called by qbank module
// key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) SubUserClaimReward(ctx sdk.Context, uid, vaultID string, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
	b := store.Get(key)
	var qcoins types.QCoins
	if b == nil {
		panic(fmt.Sprintf("claim amount is empty for the key key=%v", string(key)))
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		// Make sure that the stored coin set is in sorted order.
		// As the single coin element is always sorted, so the Add will never panic
		qcoins.Coins = qcoins.Coins.Sub(sdk.NewCoins(coin))
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

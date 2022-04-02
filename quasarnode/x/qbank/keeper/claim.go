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

// AddUserClaimDeposit adds user's claim amount. This method is called by orion vault
// key - types.UserDepositKBP + {uid}
func (k Keeper) AddUserClaimDeposit(ctx sdk.Context, uid, vaultID string, coin sdk.Coin) {
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

// SubUserClaimDeposit subs user's claim amount. This method is called by qbank module
// key - types.UserDepositKBP + {uid}
func (k Keeper) SubUserClaimDeposit(ctx sdk.Context, uid, vaultID string, coin sdk.Coin) {
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

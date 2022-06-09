package keeper

import (
	"fmt"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// GetUserClaimAmt get the current value of user's total claimable amount.
// Key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) GetUserClaimAmt(ctx sdk.Context, uid, vaultID string) (val types.QCoins, found bool) {
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

// Claim will remove users key from the KV store value for the requested user
func (k Keeper) ClaimAll(ctx sdk.Context, uid, vaultID string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
	store.Delete(key)
}

// GetAllClaimableRewards returns a list of all claimable tokens done so far for each users.
func (k Keeper) GetAllClaimableRewards(ctx sdk.Context) []types.UserBalanceInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Debug("Method", "GetAllClaimableRewards", "blockheight", ctx.BlockHeight)

	var totalClaimableRewards []types.UserBalanceInfo

	// key = {uid} + ":" + {vaultID}, value = types.QCoins
	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		splits := types.SplitKeyBytes(key)
		userAccStr := string(splits[0])
		var qcoin types.QCoins
		k.cdc.MustUnmarshal(value, &qcoin)

		userClaimableReward := types.UserBalanceInfo{Type: types.BalanceType_CLAIMABLE_REWARDS,
			VaultID:             oriontypes.ModuleName,
			DepositorAccAddress: userAccStr,
			Coins:               qcoin.Coins,
		}
		totalClaimableRewards = append(totalClaimableRewards, userClaimableReward)
	}

	logger.Debug("TotalClaimableRewards", totalClaimableRewards)
	return totalClaimableRewards
}

///////////////////////////////////////////////////////////////////////////////////
////////////// Methods for the users claimed reward  //////////////////////////////
///////////////////////////////////////////////////////////////////////////////////

// GetUserClaimedAmt get the current value of user's total claimed amount so far.
// Key - types.UserClaimedKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) GetUserClaimedAmt(ctx sdk.Context, uid, vaultID string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimedKBP)
	b := store.Get(types.CreateUsersClaimKey(uid, vaultID, types.Sep))

	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)
	return val, true
}

// AddUserClaimedRewards adds user's claimed amount in sdk.Coins, to maintain
// users total aggregated claim amount
// This method is called by message server claim method, when a successful claim is done.
// key - types.UserClaimedKBP + {userAccount} + {":"} + {VaultID}
func (k Keeper) AddUserClaimedRewards(ctx sdk.Context, uid, vaultID string, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimedKBP)
	key := types.CreateUsersClaimKey(uid, vaultID, types.Sep)
	b := store.Get(key)
	var qcoins types.QCoins
	if b == nil {
		qcoins.Coins = coins
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		for _, coin := range coins {
			qcoins.Coins = qcoins.Coins.Add(coin)
		}
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

// GetAllTotalClaimedRewards returns a list of all total withdraw tokens done so far for each users.
func (k Keeper) GetAllTotalClaimedRewards(ctx sdk.Context) []types.UserBalanceInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserClaimedKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Debug("Method", "GetAllClaimableRewards", "blockheight", ctx.BlockHeight)

	var totalClaimedRewards []types.UserBalanceInfo

	// key = {uid} + ":" + {vaultID}, value = types.QCoins
	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		splits := types.SplitKeyBytes(key)
		userAccStr := string(splits[0])
		var qcoin types.QCoins
		k.cdc.MustUnmarshal(value, &qcoin)

		userClaimedReward := types.UserBalanceInfo{Type: types.BalanceType_TOTAL_CLAIMED_REWARDS,
			VaultID:             oriontypes.ModuleName,
			DepositorAccAddress: userAccStr,
			Coins:               qcoin.Coins,
		}
		totalClaimedRewards = append(totalClaimedRewards, userClaimedReward)
	}

	logger.Debug("TotalClaimedRewards", totalClaimedRewards)
	return totalClaimedRewards
}

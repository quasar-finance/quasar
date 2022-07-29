package keeper

import (
	"fmt"

	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// SetUserDepositAmt set the current value of user's total deposit amount.
// Key - types.UserDepositKBP + {userAccount}
func (k Keeper) SetUserDepositAmt(ctx sdk.Context, uid string, val types.QCoins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	b := k.cdc.MustMarshal(&val)
	store.Set(types.CreateUserDepositKey(uid), b)
}

// GetUserDepositAmt get the current value of user's total deposit amount.
// Key - types.UserDepositKBP + {userAccount}
func (k Keeper) GetUserDepositAmt(ctx sdk.Context, uid string) (val types.QCoins, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	b := store.Get(types.CreateUserDepositKey(uid))
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// GetAllDeposits returns a map denoting all deposits of user
// Logic -
// Iterate over { types.UserDepositKBP } => { CreateUserDepositKey }
func (k Keeper) GetAllDeposits(ctx sdk.Context) map[string]sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDepositKBP)
	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	k.Logger(ctx).Info(fmt.Sprintf("GetTotalDeposits|modulename=%s|blockheight=%d",
		types.ModuleName, ctx.BlockHeight()))

	allDeposits := make(map[string]sdk.Coins)
	for ; iter.Valid(); iter.Next() {
		userAcc := string(iter.Key())
		value := iter.Value()
		var qCoins types.QCoins
		k.cdc.MustUnmarshal(value, &qCoins)
		if deposit, exist := allDeposits[userAcc]; exist {
			allDeposits[userAcc] = deposit.Add(qCoins.Coins...)
		} else {
			allDeposits[userAcc] = qCoins.Coins
		}
	}
	return allDeposits
}

// GetTotalDeposits calculates the current total active deposits.
// Note - We need to guarantee actual deposit returns to the users irrespective of IL loss.
func (k Keeper) GetTotalDeposits(ctx sdk.Context) sdk.Coins {
	allCoins := sdk.NewCoins()
	for _, deposit := range k.GetAllDeposits(ctx) {
		allCoins = allCoins.Add(deposit...)
	}
	return allCoins
}

// AddUserDeposit adds user's deposit amount with key - types.UserDepositKBP + {uid} ,
// and will aggregate the total deposited amount so far.
func (k Keeper) AddUserDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	deposit, found := k.GetUserDepositAmt(ctx, uid)
	if found {
		deposit.Coins = deposit.Coins.Add(coin)
	} else {
		deposit.Coins = sdk.NewCoins(coin)
	}
	k.SetUserDepositAmt(ctx, uid, deposit)
}

// SubUserDeposit subs user's deposit amount with key - types.UserDepositKBP + {uid},
// and reduce the amount deposited so far.
func (k Keeper) SubUserDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	deposit, found := k.GetUserDepositAmt(ctx, uid)
	if found {
		deposit.Coins = deposit.Coins.Sub(sdk.NewCoins(coin))
	} else {
		panic(fmt.Sprintf("method SubUserDeposit |kv store does not have uid=%s", uid))
	}
	k.SetUserDepositAmt(ctx, uid, deposit)
}

// SetUserDenomDepositAmt sets user's denom deposit amount.
// Key - types.UserDenomDepositKBP + {userAccount} + {":"} + {denom}
func (k Keeper) SetUserDenomDepositAmt(ctx sdk.Context, uid, denom string, val sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserDenomDepositKBP)
	b := k.cdc.MustMarshal(&val)
	store.Set(types.CreateUserDenomDepositKey(uid, types.Sep, denom), b)
}

// GetUserDenomDepositAmt gets user's denom deposit amount.
// Key - types.UserDenomDepositKBP + {userAccount} + {":"} + {denom}
func (k Keeper) GetUserDenomDepositAmt(ctx sdk.Context, uid, denom string) (val sdk.Coin, found bool) {
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
	denom := coin.GetDenom()
	deposit, found := k.GetUserDenomDepositAmt(ctx, uid, denom)
	if found {
		deposit = deposit.Add(coin)
	} else {
		deposit = coin
	}
	k.SetUserDenomDepositAmt(ctx, uid, denom, deposit)
}

// SubUserDenomDeposit subs user's denom deposit amount with
// key - types.UserDenomDepositKBP + {uid} + {":"} + {denom}
func (k Keeper) SubUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin) {
	denom := coin.GetDenom()
	deposit, found := k.GetUserDenomDepositAmt(ctx, uid, denom)
	if found {
		deposit = deposit.Sub(coin)
	} else {
		panic(fmt.Sprintf("method SubUserDenomDeposit |kv store does not have uid=%s, denom=%s", uid, denom))
	}
	k.SetUserDenomDepositAmt(ctx, uid, denom, deposit)
}

// SetEpochLockupUserDenomDeposit sets the current value of user's denom deposit amount
// with given epoch day and lockup period which is sdk.coin specific to a given coin denom.
func (k Keeper) SetEpochLockupUserDenomDeposit(ctx sdk.Context,
	uid, denom string, epochDay uint64, lockupPeriod types.LockupTypes, val sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, denom, epochDay, lockupPeriod)

	b := k.cdc.MustMarshal(&val)
	store.Set(key, b)
}

// GetEpochLockupUserDenomDeposit get the current value of user's denom deposit amount
// with given epoch day and lockup period which is sdk.coin specific to a given coin denom.
func (k Keeper) GetEpochLockupUserDenomDeposit(ctx sdk.Context,
	uid, denom string, epochDay uint64, lockupPeriod types.LockupTypes) (val sdk.Coin, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	key := types.CreateEpochLockupUserDenomDepositKey(uid, types.Sep, denom, epochDay, lockupPeriod)

	b := store.Get(key)
	if b == nil {
		return val, false
	}
	k.cdc.MustUnmarshal(b, &val)

	return val, true
}

// GetEpochLockupDepositAllUsersAllDenoms get the current value of all users deposit amount
// with given epoch day and lockup period.
func (k Keeper) GetEpochLockupDepositAllUsersAllDenoms(ctx sdk.Context,
	epochDay uint64, lockupPeriod types.LockupTypes) map[string]sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	prefixKey := types.CreateEpochLockupUserKey(epochDay, lockupPeriod, types.Sep)

	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetEpochUserDepositAmt|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	userCoins := make(map[string]sdk.Coins)
	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		_, _, userAccStr, _, err := types.ParseEpochLockupUserDenomDepositKey(key)
		if err != nil {
			logger.Info("GetEpochLockupDepositAllUsersAllDenoms", "key", key, "error", err.Error())
			continue
		}
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		if coins, exist := userCoins[userAccStr]; exist {
			userCoins[userAccStr] = coins.Add(coin)
		} else {
			userCoins[userAccStr] = sdk.NewCoins(coin)
		}
		logger.Info(fmt.Sprintf("GetEpochUserDepositAmt|modulename=%s|blockheight=%d|prefixKey=%s|coin=%v",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey), coin))
	}
	return userCoins
}

// GetEpochLockupUserDepositAllDenoms get the current value of user's deposit amount
// with given epoch day and lockup period.
func (k Keeper) GetEpochLockupUserDepositAllDenoms(ctx sdk.Context,
	uid string, epochDay uint64, lockupPeriod types.LockupTypes) sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	lockupStr := types.LockupTypes_name[int32(lockupPeriod)]
	prefixKey := types.CreateEpochLockupUserSepKey(epochDay, lockupStr, uid, types.Sep)

	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	k.Logger(ctx).Info(fmt.Sprintf("GetEpochUserDepositAmt|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	coins := sdk.NewCoins()
	for ; iter.Valid(); iter.Next() {
		value := iter.Value()
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		coins = coins.Add(coin)
		k.Logger(ctx).Info(fmt.Sprintf("GetEpochUserDepositAmt|modulename=%s|blockheight=%d|prefixKey=%s|coin=%v",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey), coin))
	}
	return coins
}

// AddEpochLockupUserDenomDeposit adds user's denom deposit amount with
// Key - {EpochLockupUserDenomDepositKBP} + {epochDay} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func (k Keeper) AddEpochLockupUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochDay uint64, lockupPeriod types.LockupTypes) {
	deposit, found := k.GetEpochLockupUserDenomDeposit(ctx, uid, coin.GetDenom(), epochDay, lockupPeriod)
	if found {
		deposit = deposit.Add(coin)
	} else {
		deposit = coin
	}
	k.SetEpochLockupUserDenomDeposit(ctx, uid, coin.GetDenom(), epochDay, lockupPeriod, deposit)
}

// SubEpochLockupUserDenomDeposit subs user's denom deposit amount with
// Key - {EpochLockupUserDenomDepositKBP} +  ":" + {epochDay} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func (k Keeper) SubEpochLockupUserDenomDeposit(ctx sdk.Context, uid string, coin sdk.Coin, epochDay uint64, lockupPeriod types.LockupTypes) {
	denom := coin.Denom
	deposit, found := k.GetEpochLockupUserDenomDeposit(ctx, uid, denom, epochDay, lockupPeriod)
	if found {
		deposit = deposit.Sub(coin)
	} else {
		panic(fmt.Sprintf("method SubEpochLockupUserDenomDeposit |kv store does not have uid=%v, denom=%s, epochDay=%v, lockupPeriod=%v", uid, denom, epochDay, lockupPeriod))
	}
	k.SetEpochLockupUserDenomDeposit(ctx, uid, denom, epochDay, lockupPeriod, deposit)
}

// GetEpochUserDepositAmt calculates the total deposit amount of the given user on a given day
// Iterate over all lockup periods
// On iteration, key - {denom}, value = sdk.Coin. No. of iteration can be upto the number of lockup periods
func (k Keeper) GetEpochUserDepositAmt(ctx sdk.Context, epochDay uint64, userAcc string) sdk.Coins {
	allCoins := sdk.NewCoins()
	validLockupTypes := []types.LockupTypes{
		types.LockupTypes_Days_7,
		types.LockupTypes_Days_21,
		types.LockupTypes_Months_1,
		types.LockupTypes_Months_3,
	}
	for _, lockup := range validLockupTypes {
		coins := k.GetEpochLockupUserDepositAllDenoms(ctx, userAcc, epochDay, lockup)
		allCoins = allCoins.Add(coins...)
	}
	return allCoins
}

// GetTotalEpochDeposits calculates the total amount deposited on a given epoch day
// Logic - Iterate with epoch day as prefix keys
// Full key -  EpochLockupUserDenomDepositKBP + {epochDay} + {:}
func (k Keeper) GetTotalEpochDeposits(ctx sdk.Context, epochDay uint64) sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	prefixKey := types.EpochDaySepKey(epochDay, types.Sep)

	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetTotalEpochDeposits|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

	coins := sdk.NewCoins()
	for ; iter.Valid(); iter.Next() {
		value := iter.Value()
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		coins = coins.Add(coin)
		logger.Info(fmt.Sprintf("GetTotalEpochDeposits|modulename=%s|blockheight=%d|prefixKey=%s|coin=%v",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey), coin))
	}

	return coins
}

// GetAllDepositInfos prepare a list of all deposit infos,
// Method is used for export genesis.
// Full key -  {epochDay} + {:}+ {$lockupPeriods} + {:} + {userAcc} + {:} + {denom}
func (k Keeper) GetAllDepositInfos(ctx sdk.Context) []types.DepositInfo {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)

	iter := sdk.KVStorePrefixIterator(store, []byte{})
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetAllDepositInfos|modulename=%s|blockheight=%d",
		types.ModuleName, ctx.BlockHeight()))

	var depositInfos []types.DepositInfo
	for ; iter.Valid(); iter.Next() {
		// key = {epochDay} + {:} + {$lockupPeriods} + {:} + {userAcc} + {:} + {denom},
		// value = sdk.Coin marshaled
		key, value := iter.Key(), iter.Value()
		epochDay, lockupDayStr, userAccStr, _, err := types.ParseEpochLockupUserDenomDepositKey(key)
		if err != nil {
			logger.Info("GetAllDepositInfos", "key", key, "error", err.Error())
			continue
		}
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)

		di := types.DepositInfo{VaultID: oriontypes.ModuleName,
			EpochDay:            epochDay,
			LockupPeriod:        types.LockupTypes(types.LockupTypes_value[lockupDayStr]),
			DepositorAccAddress: userAccStr,
			Coin:                coin}

		depositInfos = append(depositInfos, di)
		logger.Info("DepositInfo", di)
	}

	return depositInfos
}

// GetAllActiveUserDeposits returns a map of total deposited coins currently in lockup per user.
func (k Keeper) GetAllActiveUserDeposits(ctx sdk.Context, todayEpochDay uint64) map[string]sdk.Coins {
	res := make(map[string]sdk.Coins)
	for _, depositInfo := range k.GetAllDepositInfos(ctx) {
		if !depositInfo.IsActiveOn(todayEpochDay) {
			continue
		}
		if deposits, exist := res[depositInfo.DepositorAccAddress]; exist {
			res[depositInfo.DepositorAccAddress] = deposits.Add(depositInfo.Coin)
		} else {
			res[depositInfo.DepositorAccAddress] = sdk.NewCoins(depositInfo.Coin)
		}
	}
	return res
}

// GetAllTotalDeposits prepare a list of total deposit info for each user
// Method is used for export genesis.
func (k Keeper) GetAllTotalDeposits(ctx sdk.Context) []types.UserBalanceInfo {
	var totalDepositInfos []types.UserBalanceInfo
	for userAccStr, deposit := range k.GetAllDeposits(ctx) {
		userTotalDeposit := types.UserBalanceInfo{Type: types.BalanceType_TOTAL_DEPOSIT,
			VaultID:             oriontypes.ModuleName,
			DepositorAccAddress: userAccStr,
			Coins:               deposit,
		}
		totalDepositInfos = append(totalDepositInfos, userTotalDeposit)
	}

	logger := k.Logger(ctx)
	logger.Info("GetAllTotalDeposits", "TotalDepositInfos", totalDepositInfos)
	return totalDepositInfos
}

// GetEpochLockupCoins get the todays lockup coins tuple list.
func (k Keeper) GetEpochLockupCoins(ctx sdk.Context, epochDay uint64) types.EpochLockupCoins {
	elcs := types.EpochLockupCoins{}
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.EpochLockupUserDenomDepositKBP)
	prefixKey := types.EpochDaySepKey(epochDay, types.Sep)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("GetAllEpochLockupCoins|modulename=%s|blockheight=%d|prefixKey=%s",
		types.ModuleName, ctx.BlockHeight(), string(prefixKey)))
	// key = {$lockupPeriods} + {:} + {userAcc} + {:} + {denom}
	// value = sdk.Coin marshaled
	for ; iter.Valid(); iter.Next() {
		key, value := iter.Key(), iter.Value()
		splits := types.SplitKeyBytes(key)
		lockupPeriod := types.LockupTypes(types.LockupTypes_value[string(splits[0])])
		//denomStr := string(splits[2])
		var coin sdk.Coin
		k.cdc.MustUnmarshal(value, &coin)
		elcs.Infos = append(elcs.Infos,
			types.EpochLockupCoinInfo{EpochDay: epochDay, LockupPeriod: lockupPeriod, Coin: coin})

	}
	return elcs
}

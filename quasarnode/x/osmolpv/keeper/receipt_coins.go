package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RC - Receipt Coin
// AUDIT NOTE - This could be a reduntant method
// Get the list of denoms types allocated to input address addr of type sdk.AccAddress
func (k Keeper) GetRCDenoms(ctx sdk.Context, addr sdk.AccAddress) qbanktypes.QDenoms {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserReceiptCoinsKBP)
	key := types.CreateUserReceiptCoinsKey(addr)
	b := store.Get(key)
	var qdenoms qbanktypes.QDenoms
	k.cdc.MustUnmarshal(b, &qdenoms)
	return qdenoms
}

// AUDIT NOTE - This could be a reduntant method
// Add a denom to the list of receipt token denom type allocated to a user
func (k Keeper) AddRCDenoms(ctx sdk.Context, addr sdk.AccAddress, denom string) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserReceiptCoinsKBP)
	key := types.CreateUserReceiptCoinsKey(addr)
	b := store.Get(key)
	var qdenoms qbanktypes.QDenoms
	if b == nil {
		qdenoms.Denoms = append(qdenoms.Denoms, denom)
		value := k.cdc.MustMarshal(&qdenoms)
		store.Set(key, value)
	} else {
		k.cdc.MustUnmarshal(b, &qdenoms)
		qdenoms.Denoms = append(qdenoms.Denoms, denom)
		value := k.cdc.MustMarshal(&qdenoms)
		store.Set(key, value)
	}
}

// AUDIT NOTE - This could be a reduntant method
// Retrieve the amount of receipt coins as a slice of sdk.Coin as sdk.Coins
// help by an account
func (k Keeper) GetAllRCBalances(ctx sdk.Context, addr sdk.AccAddress, denoms []string) sdk.Coins {
	var receiptCoins sdk.Coins
	for _, denom := range denoms {
		coin := k.GetRCBalance(ctx, addr, denom)
		receiptCoins = append(receiptCoins, coin)
	}
	return receiptCoins
}

// AUDIT NOTE - This could be a reduntant method
// Retrive the amount of receipt coins per denomication held by an account
func (k Keeper) GetRCBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin {
	balance := k.bankKeeper.GetBalance(ctx, addr, denom)
	return balance
}

// GetTotalOrions calculates the total amount of orions for the input sdk.Coins
// This method will be used for end user queries by the front end in two steps.
// Step #1 - Front end calls the current total users deposits as sdk.Coins to qbank module
// Step #2 - Front end calls the current Orions amount equivalent to the input sdk.Coins
// GetTotalOrions method will be used for step #2
// AUDIT TODO - grpc query to be added
func (k Keeper) GetTotalOrions(ctx sdk.Context, coins sdk.Coins) sdk.Coin {
	var orions sdk.Coin
	for _, coin := range coins {
		orion := k.CalcReceipts(ctx, coin)
		orions = orions.Add(orion)
	}
	return orions
}

// CalcReceipts calculates the amount of orion coin equivalent to the input sdk.Coin
// This method is supposed to be called by the orion module.
func (k Keeper) CalcReceipts(ctx sdk.Context, coin sdk.Coin) sdk.Coin {
	spotPrice := k.GetSpotPrice(ctx, coin.Denom)
	OrionAmt := coin.Amount.ToDec().Mul(spotPrice).TruncateInt()
	return sdk.NewCoin(types.ModuleName, OrionAmt)
}

// GetSpotPrice gets the amount of UST equivalent to the input one denom from the qoracle module
func (k Keeper) GetSpotPrice(ctx sdk.Context, denom string) sdk.Dec {
	spotPrice := k.qoracleKeeper.GetStablePrice(ctx, denom)
	return spotPrice
}

// MintOrion will mint orions from the Orion module and transfer to the orion module account.
func (k Keeper) MintOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.bankKeeper.MintCoins(ctx, types.ModuleName, sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// BurnOrion will mint orions from the Orion module and transfer to the orion module account.
func (k Keeper) BurnOrion(ctx sdk.Context, amt sdk.Int) error {
	return k.bankKeeper.BurnCoins(ctx, types.ModuleName,
		sdk.NewCoins(sdk.NewCoin(types.ModuleName, amt)))
}

// GetUsersOrionShare calculates the percentage of users Orion share based on the
// Total Orion printed so far, and total users deposit in the orion vault.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions
func (k Keeper) GetUsersOrionShare(ctx sdk.Context, userAcc string) sdk.Dec {
	qc, _ := k.qbankKeeper.GetUserDepositAmt(ctx, userAcc)
	usersOrion := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range qc.Coins {
		orion := k.CalcReceipts(ctx, c)
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetTotalActiveDeposits(ctx, types.ModuleName)
	totalOrions := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range allCoins {
		orion := k.CalcReceipts(ctx, c)
		totalOrions = totalOrions.Add(orion)
	}
	usershare := usersOrion.Amount.ToDec().QuoInt(totalOrions.Amount)

	return usershare
}

// GetEpochUsersOrionShare calculates the percentage of users Orion share based on a given epoch day
// and total users deposit in the orion vault on the same epoch day.
// Total orion = Users orion amounts + orion coin amount owned by the orion module
// Users share = users equivalent orions / total equivalent deposited orions
func (k Keeper) GetEpochUsersOrionShare(ctx sdk.Context,
	epochDay uint64, userAcc string) sdk.Dec {
	coins := k.qbankKeeper.GetEpochUserDepositAmt(ctx, epochDay, userAcc)
	usersOrion := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range coins {
		orion := k.CalcReceipts(ctx, c)
		usersOrion = usersOrion.Add(orion)
	}
	allCoins := k.qbankKeeper.GetEpochTotalActiveDeposits(ctx, epochDay, types.ModuleName)
	totalOrions := sdk.NewCoin(types.ModuleName, sdk.ZeroInt())
	for _, c := range allCoins {
		orion := k.CalcReceipts(ctx, c)
		totalOrions = totalOrions.Add(orion)
	}
	usershare := usersOrion.Amount.ToDec().QuoInt(totalOrions.Amount)

	return usershare
}

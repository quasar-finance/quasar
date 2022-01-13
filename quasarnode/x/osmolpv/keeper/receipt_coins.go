package keeper

import (
	"github.com/abag/quasarnode/x/osmolpv/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RC - Receipt Coin

// Get the list of denoms types allocated to input address addr of type sdk.AccAddress
func (k Keeper) GetRCDenoms(ctx sdk.Context, addr sdk.AccAddress) qbanktypes.QDenoms {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.UserReceiptCoinsKBP)
	key := types.CreateUserReceiptCoinsKey(addr)
	b := store.Get(key)
	var qdenoms qbanktypes.QDenoms
	k.cdc.MustUnmarshal(b, &qdenoms)
	return qdenoms
}

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

// Retrive the amount of receipt coins per denomication held by an account
func (k Keeper) GetRCBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin {
	balance := k.bankKeeper.GetBalance(ctx, addr, denom)
	return balance
}

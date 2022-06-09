package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) GetAvailableInterchainFund(ctx sdk.Context) sdk.Coins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.AvailableInterchainFundKBP)
	key := types.CreateInterchainFundKey()
	b := store.Get(key)
	var qcoins qbanktypes.QCoins
	if b == nil {
		return sdk.NewCoins()
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		return qcoins.Coins
	}
}

// AddAvailableInterchainFund to be called -
// 1. on receiving positive ack on ibc token transfer.
// 2. on receiving negative ack or timeout from join pool. (revert the state)
func (k Keeper) AddAvailableInterchainFund(ctx sdk.Context, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.AvailableInterchainFundKBP)
	key := types.CreateInterchainFundKey()
	b := store.Get(key)
	var qcoins qbanktypes.QCoins
	if b == nil {
		qcoins.Coins = qcoins.Coins.Add(coins...)
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	} else {
		k.cdc.MustUnmarshal(b, &qcoins)
		qcoins.Coins = qcoins.Coins.Add(coins...)
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

// SubAvailableInterchainFund - On join pool.
func (k Keeper) SubAvailableInterchainFund(ctx sdk.Context, coins sdk.Coins) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.AvailableInterchainFundKBP)
	key := types.CreateInterchainFundKey()
	b := store.Get(key)
	if b == nil {
		panic(fmt.Sprintf("method SubAvailableInterchainFund | kv store does not have key=%v", string(key)))
	} else {
		var qcoins qbanktypes.QCoins
		k.cdc.MustUnmarshal(b, &qcoins)
		qcoins.Coins = qcoins.Coins.Add(coins...)
		value := k.cdc.MustMarshal(&qcoins)
		store.Set(key, value)
	}
}

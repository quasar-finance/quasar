package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// OnJoinPoolAck -
// 1. Get LP position from input seq number
// 2. If ack has err then
// 	  set the status as failed.
//    and failed lp will be handled in the audit method
// 3. If ack is successful then
//    set the status as JOINED
func (k Keeper) OnJoinPoolAck(ctx sdk.Context, packetSeq uint64, err error) {
	if err != nil {
	}
}

// OnJoinPoolTimeout
// 1. Get LP position from input seq number
// 2. set the status as timedout.
//    and timedout lp will be handled in the audit method
func (k Keeper) OnJoinPoolTimeout(ctx sdk.Context, packetSeq uint64) {

}

func (k Keeper) OnExitPoolAck(ctx sdk.Context, packetSeq uint64, err error) {

	if err != nil {
	}
}

func (k Keeper) OnIBCTokenTransferAck(ctx sdk.Context, packetSeq uint64, ok bool) {
	if ok {
		coin, found := k.GetIBCTokenTransferRecord(ctx, packetSeq)
		if found {
			k.AddAvailableInterchainFund(ctx, coin)
			k.DeleteIBCTokenTransferRecord(ctx, packetSeq)
		}
	} else {

	}
}

func (k Keeper) OnIBCTokenTransferTimeout(ctx sdk.Context, packetSeq uint64) {
	// App level Retry max three times. Although retry is automatically done by the relayer.
	// Assuming that the ibc token transfer is robust and will return the requested amount.
	k.DeleteIBCTokenTransferRecord(ctx, packetSeq)
	// Should we return the amount back to user?
}

func (k Keeper) AddAvailableInterchainFund(ctx sdk.Context, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.AvailableInterchainFundKBP)
	key := types.CreateInterchainFundKey()
	b := store.Get(key)
	if b == nil {
		value := k.cdc.MustMarshal(&coin)
		store.Set(key, value)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

func (k Keeper) SubAvailableInterchainFund(ctx sdk.Context, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.AvailableInterchainFundKBP)
	key := types.CreateInterchainFundKey()
	b := store.Get(key)
	if b == nil {
		panic(fmt.Sprintf("method SubAvailableInterchainFund | kv store does not have key=%v", string(key)))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

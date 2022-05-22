package keeper

import (
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
		}
	}
}

func (k Keeper) OnIBCTokenTransferTimeout(ctx sdk.Context, packetSeq uint64) {
	// App level Retry max three times. Although retry is automatically done by the relayer.
	//
}

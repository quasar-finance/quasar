package keeper

import (
	"fmt"
	"time"

	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
)

// getOwnerAccStr returns the module account bech32 with which ICA transactions to be done
func (k Keeper) getOwnerAcc() sdk.AccAddress {
	return k.accountKeeper.GetModuleAddress(types.ModuleName)
}

// getOwnerAccStr returns the module account bech32 with which ICA transactions to be done
func (k Keeper) getOwnerAccStr() string {
	accAddr := k.accountKeeper.GetModuleAddress(types.ModuleName)
	accStr, err := sdk.Bech32ifyAddressBytes("quasar", accAddr)
	if err != nil {
		panic(err)
	}
	return accStr
}

func (k Keeper) GetDestinationChainId(ctx sdk.Context) string {
	return k.DestinationChainId(ctx)
}

func (k Keeper) getDestinationLocalZoneId(ctx sdk.Context) string {
	return k.OsmosisLocalInfo(ctx).LocalZoneId
}

// getConnectionId returns the connection identifier to osmosis from intergamm module
// func (k Keeper) GetConnectionId(ctx sdk.Context, chainID string) (string, bool) {
func (k Keeper) GetConnectionId(ctx sdk.Context) (string, bool) {
	if k.OsmosisLocalInfo(ctx).ConnectionId == "" {
		return "", false
	}
	return k.OsmosisLocalInfo(ctx).ConnectionId, true
}

// Intergamm module method wrappers
func (k Keeper) JoinPool(ctx sdk.Context, poolID uint64, shareOutAmount sdk.Int, tokenInMaxs []sdk.Coin) (uint64, error) {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|poolID=%v|shareOutAmount=%v|tokenInMaxs=%v\n",
		poolID, shareOutAmount, tokenInMaxs))

	owner := k.getOwnerAccStr()
	connectionId, found := k.GetConnectionId(ctx)
	if !found {
		return 0, fmt.Errorf("join pool failed due to connection id not found for ica message")
	}
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()
	packetSeq, _, _, err := k.intergammKeeper.TransmitIbcJoinPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareOutAmount,
		tokenInMaxs,
	)
	return packetSeq, err
}

func (k Keeper) LockLPTokens(ctx sdk.Context,
	duration time.Duration,
	coins sdk.Coins) (uint64, error) {

	owner := k.getOwnerAccStr()
	connectionId, found := k.GetConnectionId(ctx)
	if !found {
		return 0, fmt.Errorf("lock tokens failed due to connection id not found for ica message")
	}
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()
	packetSeq, _, _, err := k.intergammKeeper.TransmitIbcLockTokens(ctx,
		owner, connectionId, uint64(timeoutTimestamp), duration, coins)

	return packetSeq, err
}

func (k Keeper) ExitPool(ctx sdk.Context, poolID uint64, shareInAmount sdk.Int, tokenOutMins []sdk.Coin) (uint64, error) {

	k.Logger(ctx).Info("Entered JoinPool",
		"PoolID", poolID,
		"shareInAmount", shareInAmount,
		"tokenOutMins", tokenOutMins)

	owner := k.getOwnerAccStr()
	connectionId, found := k.GetConnectionId(ctx)
	if !found {
		return 0, fmt.Errorf("exit pool failed due to connection id not found for ica message")
	}
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()
	seq, _, _, err := k.intergammKeeper.TransmitIbcExitPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareInAmount,
		tokenOutMins,
	)
	return seq, err
}

// TokenWithdrawFromOsmosis initiate the token transfer from the osmosis to quasar
// via middle chain using packet forwarder.
func (k Keeper) TokenWithdrawFromOsmosis(ctx sdk.Context, coin sdk.Coin) (uint64, error) {
	k.Logger(ctx).Info("TokenWithdrawFromOsmosis", "coin", coin)
	owner := k.getOwnerAccStr()
	receiverAddr := k.getOwnerAccStr() // receiver is same as owner address

	seq, _, _, err :=  k.intergammKeeper.TransmitICATransfer(
		ctx,
		owner,
		uint64(ctx.BlockTime().Add(time.Minute).UnixNano()),
		coin,
		receiverAddr,
		ibcclienttypes.ZeroHeight(),
		uint64(ctx.BlockTime().Add(2*time.Minute).UnixNano()),
	)
	return seq, err
}

// IBCTokenTransfer does the multi hop token transfer to the osmosis interchain account via middle chain.
// Returns the packet sequence number of the outgoing packet.
// Logic - if denom is ibc atom then fwd it via cosmos-hub, and so on.
// All these details is supposed to be available in a generic place independent of orion module.
// Intergamm module should be intelligent enough to route packets based on denoms.
// The best orion or any other module can provide to intergamm is to give denom string.
// Assumption - IBC token transfer is robust enough to deal with failure. It will return tokens
// in case of failure.
// Can we actually query or determine if ibc token transfer call was successful. And if it failed; tokens
// are returned.
func (k Keeper) IBCTokenTransfer(ctx sdk.Context, coin sdk.Coin) (uint64, error) {
	logger := k.Logger(ctx)
	logger.Info("IBCTokenTransfer",
		"coin", coin,
	)
	_, found := k.intergammKeeper.IsICACreatedOnDenomNativeZone(ctx, coin.Denom, k.getOwnerAccStr())
	if !found {
		err := fmt.Errorf("error: orion ICA for orion address %s not found on native zone for denom '%s'", k.getOwnerAccStr(), coin.Denom)
		logger.Error("IBCTokenTransfer", err.Error())
		return 0, err
	}
	destAccStr, found := k.intergammKeeper.IsICACreatedOnZoneId(ctx, intergammtypes.OsmosisZoneId, k.getOwnerAccStr())
	if !found {
		err := fmt.Errorf("error: orion ICA for orion address %s not found on osmosis zone", k.getOwnerAccStr())
		logger.Error("IBCTokenTransfer", err.Error())
		return 0, err
	}

	seqNo, _, _, err := k.intergammKeeper.SendToken(ctx,
		intergammtypes.OsmosisZoneId,
		k.getOwnerAcc(),
		destAccStr,
		coin)
	logger.Debug("IBCTokenTransfer", "seqNo: ", seqNo)
	if err != nil {
		logger.Error("IBCTokenTransfer", err.Error())
	}
	ibcTransferRecord := types.IbcTokenTransfer{SeqNo: seqNo,
		Destination: k.getDestinationLocalZoneId(ctx),
		Sender:      k.getOwnerAccStr(),
		Receiver:    destAccStr,
		StartTime:   time.Now().UTC(),
		EpochDay:    uint64(k.epochsKeeper.GetEpochInfo(ctx, "day").CurrentEpoch),
		Coin:        coin,
	}

	k.SetIBCTokenTransferRecord(ctx, ibcTransferRecord)
	return seqNo, err
}

func (k Keeper) SetIBCTokenTransferRecord(ctx sdk.Context, ibcTokenTransfer types.IbcTokenTransfer) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferKBP)
	key := types.CreateSeqKey(ibcTokenTransfer.SeqNo)
	value := k.cdc.MustMarshal(&ibcTokenTransfer)
	store.Set(key, value)
}

// DeleteIBCTokenTransferRecord should be called when we receive positive ack.
// It means that we are done with the seq number successfully
func (k Keeper) DeleteIBCTokenTransferRecord(ctx sdk.Context, seqNo uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferKBP)
	key := types.CreateSeqKey(seqNo)
	store.Delete(key)
}

////////////
func (k Keeper) SetIBCTokenTransferRecord2(ctx sdk.Context,
	seqNo uint64,
	e qbanktypes.EpochLockupCoinInfo) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferSentKBP)
	key := types.CreateSeqKey(seqNo)
	value := k.cdc.MustMarshal(&e)
	store.Set(key, value)
}

func (k Keeper) DeleteIBCTokenTransferRecord2(ctx sdk.Context, seqNo uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferSentKBP)
	key := types.CreateSeqKey(seqNo)
	store.Delete(key)
}

func (k Keeper) GetIBCTokenTransferRecord(ctx sdk.Context, seqNo uint64) (ibokenTransfer types.IbcTokenTransfer, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferKBP)
	key := types.CreateSeqKey(seqNo)
	b := store.Get(key)
	if b == nil {
		return ibokenTransfer, false
	}
	k.cdc.MustUnmarshal(b, &ibokenTransfer)
	return ibokenTransfer, true
}

func (k Keeper) GetIBCTokenTransferRecord2(ctx sdk.Context,
	seqNo uint64) (e qbanktypes.EpochLockupCoinInfo, found bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferSentKBP)
	key := types.CreateSeqKey(seqNo)
	b := store.Get(key)
	if b == nil {
		return e, false
	}
	k.cdc.MustUnmarshal(b, &e)
	return e, true
}

func (k Keeper) GetTotalEpochTransffered(ctx sdk.Context, epochNumber uint64) sdk.Coins {
	return nil
}

func (k Keeper) GetTransferredEpochLockupCoins(ctx sdk.Context, epochDay uint64) qbanktypes.EpochLockupCoins {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferredKBP)
	es := []qbanktypes.EpochLockupCoinInfo{}
	prefixKey := qbanktypes.EpochDaySepKey(epochDay, types.Sep)
	iter := sdk.KVStorePrefixIterator(store, prefixKey)
	defer iter.Close()

	// key -> lockupStr, value -> qbanktypes.EpochLockupCoinInfo
	for ; iter.Valid(); iter.Next() {
		_, value := iter.Key(), iter.Value()
		var info qbanktypes.EpochLockupCoinInfo
		k.cdc.MustUnmarshal(value, &info)
		es = append(es, info)
	}
	e := qbanktypes.EpochLockupCoins{Infos: es}
	return e
}

func (k Keeper) SetTransferredEpochLockupCoins(ctx sdk.Context,
	e qbanktypes.EpochLockupCoinInfo) {

	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.IBCTokenTransferredKBP)
	key := types.CreateEpochLockupKey(e.EpochDay, e.LockupPeriod)
	value := k.cdc.MustMarshal(&e)
	store.Set(key, value)

	// get details of epoch, lockup , coins from seqNo.

	// set total epoch transfer values
	// <k,v> -> <epoch/lockup , sdk.Coins>
	// <k1,v1> -> <epoch/denom , sdk.Coin>
}

// Token withdraw from osmosis
func (k Keeper) SetSeqTokenWithdrawFromOsmosis(ctx sdk.Context, iw types.IbcIcaWithdraw) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqTokenWithdrawFromOsmosisKBP)
	key := types.CreateSeqKey(iw.SeqNo)
	value := k.cdc.MustMarshal(&iw)
	store.Set(key, value)
}

func (k Keeper) GetSeqTokenWithdrawFromOsmosis(ctx sdk.Context, seqNo uint64) types.IbcIcaWithdraw {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqTokenWithdrawFromOsmosisKBP)
	key := types.CreateSeqKey(seqNo)
	bz := store.Get(key)
	var iw types.IbcIcaWithdraw
	k.cdc.MustUnmarshal(bz, &iw)
	return iw
}

func (k Keeper) DeleteSeqTokenWithdrawFromOsmosis(ctx sdk.Context, seqNo uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.SeqTokenWithdrawFromOsmosisKBP)
	key := types.CreateSeqKey(seqNo)
	store.Delete(key)
}

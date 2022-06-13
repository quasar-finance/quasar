package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
)

func (k Keeper) TransferIbcTokens(
	ctx sdk.Context,
	srcPort, srcChannel string,
	token sdk.Coin,
	sender sdk.AccAddress,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) (uint64, error) {
	seq, found := k.channelKeeper.GetNextSequenceSend(ctx, srcPort, srcChannel)
	if !found {
		return 0, sdkerrors.Wrapf(
			channeltypes.ErrSequenceSendNotFound,
			"source port: %s, source channel: %s", srcPort, srcChannel,
		)
	}

	err := k.ibcTransferKeeper.SendTransfer(
		ctx,
		srcPort,
		srcChannel,
		token,
		sender,
		receiver,
		timeoutHeight,
		timeoutTimestamp,
	)
	if err != nil {
		return 0, err
	}
	return seq, nil
}

func (k Keeper) ForwardTransferIbcTokens(
	ctx sdk.Context,
	srcPort, srcChannel string,
	token sdk.Coin,
	sender sdk.AccAddress,
	fwdTransferPort, fwdTransferChannel string,
	intermediateReceiver string,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) (uint64, error) {
	fwdReceiver := buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)

	return k.TransferIbcTokens(
		ctx,
		srcPort,
		srcChannel,
		token,
		sender,
		fwdReceiver,
		timeoutHeight,
		timeoutTimestamp,
	)
}

func (k Keeper) TransmitIbcTransfer(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort, transferChannel string,
	token sdk.Coin,
	receiver string,
	transferTimeoutHeight ibcclienttypes.Height,
	transferTimeoutTimestamp uint64) (uint64, error) {
	iaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: connectionId,
	})
	if err != nil {
		return 0, err
	}

	msgs := []sdk.Msg{
		&ibctransfertypes.MsgTransfer{
			SourcePort:       transferPort,
			SourceChannel:    transferChannel,
			Token:            token,
			Sender:           iaResp.InterchainAccountAddress,
			Receiver:         receiver,
			TimeoutHeight:    transferTimeoutHeight,
			TimeoutTimestamp: transferTimeoutTimestamp,
		},
	}
	return k.sendTxOverIca(ctx, owner, connectionId, msgs, timeoutTimestamp)
}

// TransmitForwardIbcTransfer sends a special case of ibc transfer message that will be forwarded to the destination chain through a middle chain.
// fwdTransferPort and fwdTransferChannel are the port and channel to destination chain on the middle chain and intermidateReceiver this an account at the middle chain
// that receives the token temporarily which then sends the token to receiver on destination chain via another ibc transfer packet.
// Note that the middle chain must support packet forward wrapper module (https://github.com/strangelove-ventures/packet-forward-middleware).
func (k Keeper) TransmitForwardIbcTransfer(
	ctx sdk.Context,
	owner string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort, transferChannel string,
	token sdk.Coin,
	fwdTransferPort, fwdTransferChannel string,
	intermediateReceiver string,
	receiver string,
	transferTimeoutHeight ibcclienttypes.Height,
	transferTimeoutTimestamp uint64) (uint64, error) {

	fwdReceiver := buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)
	return k.TransmitIbcTransfer(
		ctx,
		owner,
		connectionId,
		timeoutTimestamp,
		transferPort, transferChannel,
		token,
		fwdReceiver,
		transferTimeoutHeight,
		transferTimeoutTimestamp,
	)
}

// TODO - TO be replaced with upcoming token transfer wrapper.
// Send method determin the routing logic for the coin from the caller.
// Routing logic is based on the denom and destination chain.
// Ex.
// 1. If denom is ibc atom and dest chain is osmosis, multihop token xfer to osmosis via cosmos-hub.
// 2. If denom is ibc osmos and dest chain is osmosis, do ibc token xfer to osmosis
// It will get the details from the params for each whitelisted denoms
// Send method also need to calculate the intermediate address
func (k Keeper) Send(ctx sdk.Context,
	coin sdk.Coin,
	destinationChain string,
	owner string,
	destinationAddress string) (uint64, error) {

	// TODO - Routing logic to be written here
	// Assuming that it is ibc atom
	connectionTimeout := uint64(ctx.BlockTime().UnixNano()) + DefaultSendTxRelativeTimeoutTimestamp
	transferTimeoutHeight := ibcclienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	return k.TransmitForwardIbcTransfer(ctx,
		owner,
		"connection-0",
		connectionTimeout,
		"transfer",
		"channel-0",
		coin,
		"transfer",
		"channel-0",
		"cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu", // alice on hub, maybe we can have a dedicated cosmos-hub interchain account for orion.
		destinationAddress,
		transferTimeoutHeight,
		connectionTimeout,
	)
}

// buildPacketForwardReceiver builds the receiver address for packet forward transfer based on the format below:
// {intermediate_refund_address}|{foward_port}/{forward_channel}:{final_destination_address}
func buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver string) string {
	return fmt.Sprintf("%s|%s/%s:%s", intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)
}

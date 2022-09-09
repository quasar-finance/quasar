package keeper

import (
	"errors"
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
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

// TransmitICATransferGeneral sends an ICA transfer message that may be forwarded to the destination chain through a middle chain.
// Note that the middle chain must support packet forward wrapper module (https://github.com/strangelove-ventures/packet-forward-middleware).
func (k Keeper) TransmitICATransferGeneral(
	ctx sdk.Context,
	owner string,
	icaZoneId string,
	msgTransmitTimeoutTimestamp uint64,
	token sdk.Coin,
	dstZoneId string,
	finalReceiver string,
	icaTransferTimeoutHeight ibcclienttypes.Height,
	icaTransferTimeoutTimestamp uint64) (uint64, error) {
	logger := k.Logger(ctx)

	icaZoneInfo, found := k.GetZoneInfo(ctx, icaZoneId)
	if !found {
		msg := fmt.Sprintf("error: destination zone info for zone ID '%s' not found in CompleteZoneInfoMap for direct transfer of %s",
			icaZoneId, token.String())
		logger.Error("SendToken", msg)
		return 0, errors.New(msg)
	}

	qsrToIcaConnectionId := icaZoneInfo.ZoneRouteInfo.ConnectionId
	icaResp, err := k.InterchainAccountFromAddress(sdk.WrapSDKContext(ctx), &types.QueryInterchainAccountFromAddressRequest{
		Owner:        owner,
		ConnectionId: icaZoneInfo.ZoneRouteInfo.ConnectionId,
	})
	if err != nil {
		return 0, err
	}

	nativeZoneId, found := k.DenomToNativeZoneIdMap(ctx)[token.Denom]
	if !found {
		logger.Error("TransmitICATransferGeneral", fmt.Sprintf("error: native zone ID of denom '%s' not specified", token.Denom))
		return 0, errors.New("error: unsupported denom")
	}
	nativeZoneInfo, found := k.GetZoneInfo(ctx, nativeZoneId)
	if !found {
		logger.Error("TransmitICATransferGeneral", fmt.Sprintf("error: zone info for zone ID '%s' not specified", nativeZoneId))
		return 0, errors.New("error: zone info not found")
	}

	// prepare the ICA transfer message
	var msgs []sdk.Msg
	if nativeZoneId == icaZoneId || nativeZoneId == dstZoneId {
		// direct ICA transfer

		// need to reach quasar zone from ICA zone
		icaToQsrPortId := icaZoneInfo.ZoneRouteInfo.CounterpartyPortId
		icaToQsrChannelId := icaZoneInfo.ZoneRouteInfo.CounterpartyChannelId

		msgs = append(msgs, &ibctransfertypes.MsgTransfer{
			SourcePort:       icaToQsrPortId,
			SourceChannel:    icaToQsrChannelId,
			Token:            token,
			Sender:           icaResp.InterchainAccountAddress,
			Receiver:         finalReceiver,
			TimeoutHeight:    icaTransferTimeoutHeight,
			TimeoutTimestamp: icaTransferTimeoutTimestamp,
		})
	} else {
		// forwarding ICA transfer

		// icaFromNativeInfo contains IBC info about the channel between ICA zone and the native zone.
		icaFromNativeInfo, found := nativeZoneInfo.NextZoneRouteMap[icaZoneId]
		if !found {
			msg := fmt.Sprintf("error: ICA zone info for zone ID '%s' not found in NextZoneRouteMap of native zone with ID '%s' for forwarding transfer of %s",
				icaZoneId, nativeZoneInfo.ZoneRouteInfo.CounterpartyZoneId, token.String())
			logger.Error("SendToken", msg)
			return 0, errors.New(msg)
		}

		nativeIcaAddr, found := k.IsICARegistered(ctx, nativeZoneInfo.ZoneRouteInfo.ConnectionId, owner)
		if !found {
			msg := fmt.Sprintf("error: interchain account owned by %s on native zone (zone ID '%s') for forwarding transfer of %s not found",
				owner, nativeZoneId, token.String())
			logger.Error("SendToken", msg)
			return 0, errors.New(msg)
		}

		// The fund should first go to the native zone
		icaToNativePortId := icaFromNativeInfo.CounterpartyPortId
		icaToNativeChannelId := icaFromNativeInfo.CounterpartyChannelId
		nativeToQsrPortId := nativeZoneInfo.ZoneRouteInfo.CounterpartyPortId
		nativeToQsrChannelId := nativeZoneInfo.ZoneRouteInfo.CounterpartyChannelId
		receiverAddr := buildPacketForwardReceiver(nativeIcaAddr, nativeToQsrPortId, nativeToQsrChannelId, finalReceiver)

		msgs = append(msgs, &ibctransfertypes.MsgTransfer{
			SourcePort:       icaToNativePortId,
			SourceChannel:    icaToNativeChannelId,
			Token:            token,
			Sender:           icaResp.InterchainAccountAddress,
			Receiver:         receiverAddr,
			TimeoutHeight:    icaTransferTimeoutHeight,
			TimeoutTimestamp: icaTransferTimeoutTimestamp,
		})
	}
	// transmit the ICA transfer message
	qsrToIcaPortId := icaZoneInfo.ZoneRouteInfo.PortId
	qsrToIcaChannelId := icaZoneInfo.ZoneRouteInfo.ChannelId
	return k.sendTxOverIca2(ctx, qsrToIcaConnectionId, qsrToIcaPortId, qsrToIcaChannelId, msgs, msgTransmitTimeoutTimestamp)
}

// buildPacketForwardReceiver builds the receiver address for packet forward transfer based on the format below:
// {intermediate_refund_address}|{forward_port}/{forward_channel}:{final_destination_address}
func buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver string) string {
	return fmt.Sprintf("%s|%s/%s:%s", intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)
}

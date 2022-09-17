package keeper

import (
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

// TransmitICATransferGeneral sends an ICA transfer message that may be forwarded to quasar through a middle chain.
// Note that the middle chain must support packet forward wrapper module (https://github.com/strangelove-ventures/packet-forward-middleware).
// Scope - To be used to create ibc token transfer/forward tx in the other zones connected to quasar.
// The interchain account mechanism is used to execute tx packets to the other chain.
// The token transfer/forward message formation is done on the quasar chain,
// while the tx happens on the other chain upon receiving the packet over the ICS-27 protocol standard.
func (k Keeper) TransmitICATransferGeneral(
	ctx sdk.Context,
	owner string,
	icaZoneId string,
	msgTransmitTimeoutTimestamp uint64,
	token sdk.Coin,
	finalReceiver string,
	icaTransferTimeoutHeight ibcclienttypes.Height,
	icaTransferTimeoutTimestamp uint64) (uint64, error) {
	logger := k.Logger(ctx)

	if _, err := sdk.AccAddressFromBech32(finalReceiver); err != nil {
		return 0, sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid final receiver address (%s) for quasar zone", err)
	}

	icaZoneInfo, found := k.GetZoneInfo(ctx, icaZoneId)
	if !found {
		err := sdkerrors.Wrapf(types.ErrZoneInfoNotFound, "destination zone info for zone ID '%s' not found in CompleteZoneInfoMap for direct transfer of %s",
			icaZoneId, token.String())
		logger.Error("SendToken", err)
		return 0, err
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
		err := sdkerrors.Wrapf(types.ErrDenomNativeZoneIdNotFound, "native zone ID of denom '%s' not specified", token.Denom)
		logger.Error("TransmitICATransferGeneral", err)
		return 0, err
	}

	// prepare the ICA transfer message
	var msgs []sdk.Msg
	if nativeZoneId == icaZoneId || nativeZoneId == types.QuasarZoneId {
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

		nativeZoneInfo, found := k.GetZoneInfo(ctx, nativeZoneId)
		if !found {
			err := sdkerrors.Wrapf(types.ErrZoneInfoNotFound, "zone info for zone ID '%s' not specified", nativeZoneId)
			logger.Error("TransmitICATransferGeneral", err)
			return 0, err
		}

		// icaFromNativeInfo contains IBC info about the channel between ICA zone and the native zone.
		icaFromNativeInfo, found := nativeZoneInfo.NextZoneRouteMap[icaZoneId]
		if !found {
			err := sdkerrors.Wrapf(types.ErrZoneInfoNotFound, "ICA zone info for zone ID '%s' not specified in NextZoneRouteMap of zone '%s' (native zone of %s)",
				icaZoneId, nativeZoneInfo.ZoneRouteInfo.CounterpartyZoneId, token.String())
			logger.Error("SendToken", err)
			return 0, err
		}

		nativeIcaAddr, found := k.IsICARegistered(ctx, nativeZoneInfo.ZoneRouteInfo.ConnectionId, owner)
		if !found {
			err := sdkerrors.Wrapf(types.ErrICANotFound, "no inter-chain account owned by %s found on zone '%s' (native zone of %s)",
				owner, nativeZoneId, token.String())
			logger.Error("SendToken", err)
			return 0, err
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
	return k.sendTxOverIca(ctx, owner, qsrToIcaConnectionId, msgs, msgTransmitTimeoutTimestamp)
}

// buildPacketForwardReceiver builds the receiver address for packet forward transfer based on the format below:
// {intermediate_refund_address}|{forward_port}/{forward_channel}:{final_destination_address}
func buildPacketForwardReceiver(intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver string) string {
	return fmt.Sprintf("%s|%s/%s:%s", intermediateReceiver, fwdTransferPort, fwdTransferChannel, receiver)
}

package keeper

import (
	"errors"
	"fmt"

	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
)

// TransmitIbcExitPoolPacket transmits the packet over IBC with the specified source port and source channel
func (k Keeper) TransmitIbcExitPoolPacket(
	ctx sdk.Context,
	packetData types.IbcExitPoolPacketData,
	sourcePort,
	sourceChannel string,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) error {
	k.Logger(ctx).Info(fmt.Sprintf("Entered TransmitIbcExitPoolPacket|packetData=%v|sourcePort=%v|sourceChannel=%v|timeoutHeight=%v|timeoutTimestamp=%v|\n",
		packetData, sourcePort, sourceChannel, timeoutHeight, timeoutTimestamp))
	sourceChannelEnd, found := k.ChannelKeeper.GetChannel(ctx, sourcePort, sourceChannel)
	if !found {
		return sdkerrors.Wrapf(channeltypes.ErrChannelNotFound, "port ID (%s) channel ID (%s)", sourcePort, sourceChannel)
	}

	destinationPort := sourceChannelEnd.GetCounterparty().GetPortID()
	destinationChannel := sourceChannelEnd.GetCounterparty().GetChannelID()

	// get the next sequence
	sequence, found := k.ChannelKeeper.GetNextSequenceSend(ctx, sourcePort, sourceChannel)
	if !found {
		return sdkerrors.Wrapf(
			channeltypes.ErrSequenceSendNotFound,
			"source port: %s, source channel: %s", sourcePort, sourceChannel,
		)
	}

	channelCap, ok := k.ScopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(sourcePort, sourceChannel))
	if !ok {
		return sdkerrors.Wrap(channeltypes.ErrChannelCapabilityNotFound, "module does not own channel capability")
	}

	packetBytes, err := packetData.GetBytes()
	if err != nil {
		return sdkerrors.Wrap(sdkerrors.ErrJSONMarshal, "cannot marshal the packet: "+err.Error())
	}

	packet := channeltypes.NewPacket(
		packetBytes,
		sequence,
		sourcePort,
		sourceChannel,
		destinationPort,
		destinationChannel,
		timeoutHeight,
		timeoutTimestamp,
	)
	k.Logger(ctx).Info(fmt.Sprintf("TransmitIbcExitPoolPacket|packet=%v|\n", packet))

	if err := k.ChannelKeeper.SendPacket(ctx, channelCap, packet); err != nil {
		return err
	}

	return nil
}

// OnRecvIbcExitPoolPacket processes packet reception
func (k Keeper) OnRecvIbcExitPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcExitPoolPacketData) (packetAck types.IbcExitPoolPacketAck, err error) {
	// validate packet data upon receiving
	if err := data.ValidateBasic(); err != nil {
		return packetAck, err
	}

	// TODO: packet reception logic

	return packetAck, nil
}

// OnAcknowledgementIbcExitPoolPacket responds to the the success or failure of a packet
// acknowledgement written on the receiving chain.
func (k Keeper) OnAcknowledgementIbcExitPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcExitPoolPacketData, ack channeltypes.Acknowledgement) error {
	k.Logger(ctx).Info(fmt.Sprintf("OnAcknowledgementIbcExitPoolPacket|packet=%v|data=%v|ack=%v\n", packet, data, ack))

	switch dispatchedAck := ack.Response.(type) {
	case *channeltypes.Acknowledgement_Error:
		k.Logger(ctx).Info("OnAcknowledgementIbcExitPoolPacket|Acknowledgement_Error")

		// TODO: failed acknowledgement logic
		_ = dispatchedAck.Error

		return nil
	case *channeltypes.Acknowledgement_Result:
		// Decode the packet acknowledgment
		var packetAck types.IbcExitPoolPacketAck

		if err := types.ModuleCdc.UnmarshalJSON(dispatchedAck.Result, &packetAck); err != nil {

			// The counter-party module doesn't implement the correct acknowledgment format
			return errors.New("cannot unmarshal acknowledgment")
		}
		k.Logger(ctx).Info(fmt.Sprintf("OnAcknowledgementIbcExitPoolPacket|packetAck=%v|\n", packetAck))

		// TODO: successful acknowledgement logic

		return nil
	default:
		// The counter-party module doesn't implement the correct acknowledgment format
		return errors.New("invalid acknowledgment format")
	}
}

// OnTimeoutIbcExitPoolPacket responds to the case where a packet has not been transmitted because of a timeout
func (k Keeper) OnTimeoutIbcExitPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcExitPoolPacketData) error {

	// TODO: packet timeout logic
	k.Logger(ctx).Info(fmt.Sprintf("OnTimeoutIbcExitPoolPacket|packet=%v|data=%v|\n", packet, data))

	return nil
}

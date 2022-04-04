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

// TransmitIbcJoinPoolPacket transmits the packet over IBC with the specified source port and source channel
func (k Keeper) TransmitIbcJoinPoolPacket(
	ctx sdk.Context,
	packetData types.IbcJoinPoolPacketData,
	sourcePort,
	sourceChannel string,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) error {

	k.Logger(ctx).Info(fmt.Sprintf("Entered TransmitIbcJoinPoolPacket|packetData=%v|sourcePort=%v|sourceChannel=%v|timeoutHeight=%v|timeoutTimestamp=%v|\n",
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
	k.Logger(ctx).Info(fmt.Sprintf("TransmitIbcJoinPoolPacket|packet=%v|\n", packet))
	if err := k.ChannelKeeper.SendPacket(ctx, channelCap, packet); err != nil {
		return err
	}

	return nil
}

// OnRecvIbcJoinPoolPacket processes packet reception
func (k Keeper) OnRecvIbcJoinPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcJoinPoolPacketData) (packetAck types.IbcJoinPoolPacketAck, err error) {
	// validate packet data upon receiving
	if err := data.ValidateBasic(); err != nil {
		return packetAck, err
	}

	// TODO: packet reception logic

	return packetAck, nil
}

// OnAcknowledgementIbcJoinPoolPacket responds to the the success or failure of a packet
// acknowledgement written on the receiving chain.
func (k Keeper) OnAcknowledgementIbcJoinPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcJoinPoolPacketData, ack channeltypes.Acknowledgement) error {

	k.Logger(ctx).Info(fmt.Sprintf("OnAcknowledgementIbcJoinPoolPacket|packet=%v|data=%v|ack=%v\n", packet, data, ack))

	switch dispatchedAck := ack.Response.(type) {
	case *channeltypes.Acknowledgement_Error:
		k.Logger(ctx).Info("OnAcknowledgementIbcJoinPoolPacket|Acknowledgement_Error")

		// TODO: failed acknowledgement logic
		_ = dispatchedAck.Error

		return nil
	case *channeltypes.Acknowledgement_Result:
		// Decode the packet acknowledgment
		var packetAck types.IbcJoinPoolPacketAck

		if err := types.ModuleCdc.UnmarshalJSON(dispatchedAck.Result, &packetAck); err != nil {
			// The counter-party module doesn't implement the correct acknowledgment format
			return errors.New("cannot unmarshal acknowledgment")
		}
		k.Logger(ctx).Info(fmt.Sprintf("OnAcknowledgementIbcJoinPoolPacket|packetAck=%v|\n", packetAck))

		// TODO: successful acknowledgement logic

		return nil
	default:
		// The counter-party module doesn't implement the correct acknowledgment format
		return errors.New("invalid acknowledgment format")
	}
}

// OnTimeoutIbcJoinPoolPacket responds to the case where a packet has not been transmitted because of a timeout
func (k Keeper) OnTimeoutIbcJoinPoolPacket(ctx sdk.Context, packet channeltypes.Packet, data types.IbcJoinPoolPacketData) error {

	// TODO: packet timeout logic
	k.Logger(ctx).Info(fmt.Sprintf("OnTimeoutIbcJoinPoolPacket|packet=%v|data=%v|\n", packet, data))

	return nil
}

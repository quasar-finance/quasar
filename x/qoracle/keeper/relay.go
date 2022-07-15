package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
)

func (k Keeper) OnRecvPacket(ctx sdk.Context, packet channeltypes.Packet) ([]byte, error) {
	bandchainParams := k.BandchainParams(ctx)

	switch {
	case packet.SourcePort == types.BandchainOraclePortID && packet.DestinationChannel == bandchainParams.OracleIbcParams.AuthorizedChannel:
		return k.handleOraclePacket(ctx, packet)
	default:
		return nil, sdkerrors.Wrapf(types.ErrUnauthorizedIBCPacket, "could not find any authorized IBC packet handler for packet with destination path: %s", host.ChannelPath(packet.DestinationPort, packet.DestinationChannel))
	}
}

func (k Keeper) OnAcknowledgementPacket(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	bandchainParams := k.BandchainParams(ctx)
	osmosisParams := k.OsmosisParams(ctx)

	switch {
	case packet.SourceChannel == bandchainParams.OracleIbcParams.AuthorizedChannel:
		return k.handleOracleAcknowledgment(ctx, packet, ack)
	case packet.SourceChannel == osmosisParams.ICQParams.AuthorizedChannel:
		return k.handleOsmosisICQAcknowledgment(ctx, packet, ack)
	default:
		return sdkerrors.Wrapf(types.ErrUnauthorizedIBCPacket, "could not find any authorized IBC acknowledgment handler for packet with path: %s", host.ChannelPath(packet.SourcePort, packet.SourceChannel))
	}
}

func (k Keeper) OnTimeoutPacket(ctx sdk.Context, packet channeltypes.Packet) error {
	bandchainParams := k.BandchainParams(ctx)
	osmosisParams := k.OsmosisParams(ctx)

	switch {
	case packet.SourceChannel == bandchainParams.OracleIbcParams.AuthorizedChannel:
		return k.handleOracleTimeout(ctx, packet)
	case packet.SourceChannel == osmosisParams.ICQParams.AuthorizedChannel:
		return k.handleOsmosisICQTimeout(ctx, packet)
	default:
		return sdkerrors.Wrapf(types.ErrUnauthorizedIBCPacket, "could not find any authorized IBC timeout handler for packet with path: %s", host.ChannelPath(packet.SourcePort, packet.SourceChannel))
	}
}

func (k Keeper) createOutgoingPacket(
	ctx sdk.Context,
	sourcePort string,
	sourceChannel string,
	data []byte,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) (uint64, error) {
	sourceChannelEnd, found := k.channelKeeper.GetChannel(ctx, sourcePort, sourceChannel)
	if !found {
		return 0, sdkerrors.Wrapf(
			sdkerrors.ErrUnknownRequest,
			"unknown port %s channel %s",
			sourcePort,
			sourceChannel,
		)
	}
	destinationPort := sourceChannelEnd.GetCounterparty().GetPortID()
	destinationChannel := sourceChannelEnd.GetCounterparty().GetChannelID()

	// get the next sequence
	sequence, found := k.channelKeeper.GetNextSequenceSend(ctx, sourcePort, sourceChannel)
	if !found {
		return 0, sdkerrors.Wrapf(channeltypes.ErrSequenceSendNotFound, "failed to retrieve next sequence send for channel %s on port %s", sourceChannel, sourcePort)
	}

	chanCap, ok := k.scopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(sourcePort, sourceChannel))
	if !ok {
		return 0, sdkerrors.Wrap(channeltypes.ErrChannelCapabilityNotFound,
			"module does not own channel capability")
	}

	timeoutHeight, timeoutTimestamp, err := k.convertRelativeToAbsoluteTimeout(ctx, sourcePort, sourceChannel, timeoutHeight, timeoutTimestamp)
	if err != nil {
		return 0, err
	}

	packet := channeltypes.NewPacket(
		data,
		sequence,
		sourcePort,
		sourceChannel,
		destinationPort,
		destinationChannel,
		timeoutHeight,
		timeoutTimestamp,
	)
	if err := k.ics4Wrapper.SendPacket(ctx, chanCap, packet); err != nil {
		return 0, err
	}
	return packet.Sequence, nil
}

func (k Keeper) convertRelativeToAbsoluteTimeout(
	ctx sdk.Context,
	port string,
	channel string,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) (
	absTimeoutHeight clienttypes.Height,
	absTimeoutTimestamp uint64,
	err error,
) {
	clientId, clientState, err := k.channelKeeper.GetChannelClientState(ctx, port, channel)
	if err != nil {
		return clienttypes.ZeroHeight(), 0, err
	}

	clientHeight, ok := clientState.GetLatestHeight().(clienttypes.Height)
	if !ok {
		return clienttypes.ZeroHeight(), 0, sdkerrors.Wrapf(sdkerrors.ErrInvalidHeight, "invalid height type. expected type: %T, got: %T",
			clienttypes.Height{}, clientHeight)
	}

	if !timeoutHeight.IsZero() {
		absTimeoutHeight = clientHeight
		absTimeoutHeight.RevisionNumber += timeoutHeight.RevisionNumber
		absTimeoutHeight.RevisionHeight += timeoutHeight.RevisionHeight
	}

	consensusState, _ := k.clientKeeper.GetClientConsensusState(ctx, clientId, clientHeight)
	if timeoutTimestamp != 0 {
		// use local clock time as reference time if it is later than the
		// consensus state timestamp of the counter party chain, otherwise
		// still use consensus state timestamp as reference
		now := uint64(ctx.BlockTime().UnixNano())
		consensusStateTimestamp := consensusState.GetTimestamp()
		if now > consensusStateTimestamp {
			absTimeoutTimestamp = now + timeoutTimestamp
		} else {
			absTimeoutTimestamp = consensusStateTimestamp + timeoutTimestamp
		}
	}
	return absTimeoutHeight, absTimeoutTimestamp, nil
}

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
	case packet.SourcePort == types.BandchainOraclePortID && packet.SourceChannel == bandchainParams.OracleIbcParams.AuthorizedChannel:
		return k.handleOraclePacket(ctx, packet)
	default:
		return nil, sdkerrors.Wrapf(types.ErrUnauthorizedIBCPacket, "could not find any authorized IBC packet handler for packet with path: %s", host.ChannelPath(packet.DestinationPort, packet.DestinationChannel))
	}
}

func (k Keeper) OnAcknowledgementPacket(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	bandchainParams := k.BandchainParams(ctx)

	switch {
	case packet.SourcePort == types.BandchainOraclePortID && packet.SourceChannel == bandchainParams.OracleIbcParams.AuthorizedChannel:
		return k.handleOracleAcknowledgment(ctx, packet, ack)
	default:
		return sdkerrors.Wrapf(types.ErrUnauthorizedIBCPacket, "could not find any authorized IBC acknowledgment handler for packet with path: %s", host.ChannelPath(packet.SourcePort, packet.SourceChannel))
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

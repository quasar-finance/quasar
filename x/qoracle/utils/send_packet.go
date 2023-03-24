package utils

import (
	//	"cosmossdk.io/errors"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	clienttypes "github.com/cosmos/ibc-go/v4/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	host "github.com/cosmos/ibc-go/v4/modules/core/24-host"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

func SendPacket(
	ctx sdk.Context,
	clientKeeper types.ClientKeeper,
	ics4Wrapper porttypes.ICS4Wrapper,
	channelKeeper types.ChannelKeeper,
	scopedKeeper capabilitykeeper.ScopedKeeper,
	sourcePort string,
	sourceChannel string,
	data []byte,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) (channeltypes.Packet, error) {
	sourceChannelEnd, found := channelKeeper.GetChannel(ctx, sourcePort, sourceChannel)
	if !found {
		return channeltypes.Packet{}, sdkerrors.Wrapf(
			sdkerrors.ErrUnknownRequest,
			"unknown port %s channel %s",
			sourcePort,
			sourceChannel,
		)
	}
	destinationPort := sourceChannelEnd.GetCounterparty().GetPortID()
	destinationChannel := sourceChannelEnd.GetCounterparty().GetChannelID()

	// get the next sequence
	sequence, found := channelKeeper.GetNextSequenceSend(ctx, sourcePort, sourceChannel)
	if !found {
		return channeltypes.Packet{}, sdkerrors.Wrapf(channeltypes.ErrSequenceSendNotFound, "failed to retrieve next sequence send for channel %s on port %s", sourceChannel, sourcePort)
	}

	chanCap, ok := scopedKeeper.GetCapability(ctx, host.ChannelCapabilityPath(sourcePort, sourceChannel))
	if !ok {
		return channeltypes.Packet{}, sdkerrors.Wrap(channeltypes.ErrChannelCapabilityNotFound,
			"module does not own channel capability")
	}

	timeoutHeight, timeoutTimestamp, err := convertRelativeToAbsoluteTimeout(
		ctx,
		clientKeeper,
		channelKeeper,
		sourcePort,
		sourceChannel,
		timeoutHeight,
		timeoutTimestamp,
	)
	if err != nil {
		return channeltypes.Packet{}, err
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
	err = ics4Wrapper.SendPacket(ctx, chanCap, packet)
	return packet, err
}

func convertRelativeToAbsoluteTimeout(
	ctx sdk.Context,
	clientKeeper types.ClientKeeper,
	channelKeeper types.ChannelKeeper,
	port string,
	channel string,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) (
	absTimeoutHeight clienttypes.Height,
	absTimeoutTimestamp uint64,
	err error,
) {
	clientID, clientState, err := channelKeeper.GetChannelClientState(ctx, port, channel)
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

	consensusState, _ := clientKeeper.GetClientConsensusState(ctx, clientID, clientHeight)
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

package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/bandprotocol/bandchain-packet/obi"
	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
)

func (k Keeper) SendCoinRatesRequest(ctx sdk.Context,
	callData types.CoinRatesCallData,
	askCount, minCount, prepareGas, executeGas uint64,
	feeLimit sdk.Coins,
	timeoutHeight clienttypes.Height,
	timeoutTimestamp uint64,
) (uint64, error) {
	bandchainParams := k.BandchainParams(ctx)

	packetData := bandpacket.NewOracleRequestPacketData(
		types.CoinRatesClientIDKey,
		bandchainParams.CoinRatesScriptId,
		obi.MustEncode(callData),
		askCount,
		minCount,
		feeLimit,
		prepareGas,
		executeGas,
	)
	portId, channelId, err := bandchainParams.CheckOracleActiveChannelPath()
	if err != nil {
		return 0, err
	}
	return k.createOutgoingPacket(ctx, portId, channelId, packetData.GetBytes(), timeoutHeight, timeoutTimestamp)
}

func (k Keeper) handleOraclePacket(ctx sdk.Context, packet channeltypes.Packet) ([]byte, error) {
	var packetData bandpacket.OracleResponsePacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return nil, sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientIDKey:
		var coinRatesResult types.CoinRatesResult
		if err := obi.Decode(packetData.GetResult(), &coinRatesResult); err != nil {
			return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "cannot decode the coinRates received packet")
		}

		// TODO: handle the coinRatesResult
	default:
		return nil, sdkerrors.Wrapf(sdkerrors.ErrJSONUnmarshal, "oracle received packet not found: %s", packetData.GetClientID())
	}

	return types.ModuleCdc.MustMarshalJSON(
		bandpacket.NewOracleRequestPacketAcknowledgement(packetData.GetRequestID()),
	), nil
}

func (k Keeper) handleOracleAcknowledgment(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	if !ack.Success() {
		return sdkerrors.Wrap(types.ErrFailedAcknowledgment, "received unsuccessful oracle packet acknowledgement")
	}

	var ackData bandpacket.OracleRequestPacketAcknowledgement
	if err := types.ModuleCdc.UnmarshalJSON(ack.GetResult(), &ackData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet acknowledgement data")
	}

	var packetData bandpacket.OracleResponsePacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientIDKey:
		var coinRatesResult types.CoinRatesResult
		if err := obi.Decode(packetData.GetResult(), &coinRatesResult); err != nil {
			return sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "cannot decode the coinRates oracle acknowledgment packet")
		}

		// TODO: handle the ackData and coinRatesResult
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrJSONUnmarshal, "oracle acknowledgment packet not found: %s", packetData.GetClientID())
	}
}

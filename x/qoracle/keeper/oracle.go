package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/bandprotocol/bandchain-packet/obi"
	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
)

func (k Keeper) sendCoinRatesRequest(ctx sdk.Context, callData types.CoinRatesCallData) (uint64, error) {
	coinRatesScriptParams := k.BandchainParams(ctx).CoinRatesScriptParams

	packetData := bandpacket.NewOracleRequestPacketData(
		types.CoinRatesClientIDKey,
		coinRatesScriptParams.ScriptId,
		obi.MustEncode(callData),
		coinRatesScriptParams.AskCount,
		coinRatesScriptParams.MinCount,
		coinRatesScriptParams.FeeLimit,
		coinRatesScriptParams.PrepareGas,
		coinRatesScriptParams.ExecuteGas,
	)
	return k.sendOraclePacket(ctx, packetData)
}

func (k Keeper) sendOraclePacket(ctx sdk.Context, packetData bandpacket.OracleRequestPacketData) (uint64, error) {
	port := k.GetPort(ctx)
	ibcParams := k.BandchainParams(ctx).OracleIbcParams

	return k.createOutgoingPacket(ctx, port, ibcParams.AuthorizedChannel, packetData.GetBytes(),
		ibcParams.TimeoutHeight, ibcParams.TimeoutTimestamp)
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

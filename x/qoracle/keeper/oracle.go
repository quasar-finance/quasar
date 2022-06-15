package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/bandprotocol/bandchain-packet/obi"
	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
)

func (k Keeper) sendCoinRatesRequest(ctx sdk.Context, symbols []string, mul uint64) (uint64, error) {
	coinRatesScriptParams := k.BandchainParams(ctx).CoinRatesScriptParams

	callData := types.CoinRatesCallData{
		Symbols:    symbols,
		Multiplier: mul,
	}
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
	seq, err := k.sendOraclePacket(ctx, packetData)
	if err != nil {
		return 0, err
	}

	req := types.CoinRatesLatestRequest{
		PacketSequence: seq,
		CallData:       callData,
	}
	k.setCoinRatesLatestRequest(ctx, req)
	return seq, nil
}

func (k Keeper) sendOraclePacket(ctx sdk.Context, packetData bandpacket.OracleRequestPacketData) (uint64, error) {
	port := k.GetPort(ctx)
	ibcParams := k.BandchainParams(ctx).OracleIbcParams

	return k.createOutgoingPacket(ctx, port, ibcParams.AuthorizedChannel, packetData.GetBytes(),
		ibcParams.TimeoutHeight, ibcParams.TimeoutTimestamp)
}

// setCoinRatesLatestRequest
func (k Keeper) setCoinRatesLatestRequest(ctx sdk.Context, req types.CoinRatesLatestRequest) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyPrefix(types.CoinRatesLatestRequestKey),
		k.cdc.MustMarshal(&req))
}

func (k Keeper) getCoinRatesLatestRequest(ctx sdk.Context) (types.CoinRatesLatestRequest, error) {
	store := ctx.KVStore(k.storeKey)
	var req types.CoinRatesLatestRequest
	if err := k.cdc.Unmarshal(store.Get(types.KeyPrefix(types.CoinRatesLatestRequestKey)), &req); err != nil {
		return req, err
	}
	return req, nil
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

		k.updateCoinRatesState(ctx, packetData.GetRequestID(), coinRatesResult)
	default:
		return nil, sdkerrors.Wrapf(sdkerrors.ErrJSONUnmarshal, "oracle received packet not found: %s", packetData.GetClientID())
	}

	return types.ModuleCdc.MustMarshalJSON(
		bandpacket.NewOracleRequestPacketAcknowledgement(packetData.GetRequestID()),
	), nil
}

func (k Keeper) updateCoinRatesState(ctx sdk.Context, requestId uint64, coinRatesResult types.CoinRatesResult) error {
	req, err := k.getCoinRatesLatestRequest(ctx)
	if err != nil {
		return err
	}
	if req.GetOracleRequestId() != requestId {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "coinRates request id %d does not match the latest request id %d", requestId, req.PacketSequence)
	}

	state := types.CoinRatesState{
		PacketSequence:  req.GetPacketSequence(),
		UpdateHeight:    ctx.BlockHeight(),
		OracleRequestId: req.GetOracleRequestId(),
		Rates:           coinRatesFromSymbols(req.GetCallData().Symbols, coinRatesResult.GetRates()),
	}
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyPrefix(types.CoinRatesStateKey), k.cdc.MustMarshal(&state))
	return nil
}

func coinRatesFromSymbols(symbols []string, rates []uint64) sdk.DecCoins {
	coins := make(sdk.DecCoins, len(symbols))
	for i, symbol := range symbols {
		coins[i] = sdk.NewDecCoinFromDec(symbol, sdk.NewDec(int64(rates[i])))
	}
	return coins
}

// GetCoinRatesState returns the coinRates state
func (k Keeper) GetCoinRatesState(ctx sdk.Context) (types.CoinRatesState, error) {
	store := ctx.KVStore(k.storeKey)
	var state types.CoinRatesState
	if err := k.cdc.Unmarshal(store.Get(types.KeyPrefix(types.CoinRatesStateKey)), &state); err != nil {
		return state, err
	}
	return state, nil
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

		req, err := k.getCoinRatesLatestRequest(ctx)
		if err != nil {
			return err
		}
		if req.PacketSequence != packet.GetSequence() {
			return sdkerrors.Wrapf(types.ErrInvalidPacketSequence, "expected packet sequence %d, got %d", req.PacketSequence, packet.GetSequence())
		}

		// Update request latest state with oracle request id
		req.OracleRequestId = ackData.GetRequestID()
		k.setCoinRatesLatestRequest(ctx, req)
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrJSONUnmarshal, "oracle acknowledgment packet not found: %s", packetData.GetClientID())
	}
}

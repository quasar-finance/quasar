package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/bandprotocol/bandchain-packet/obi"
	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
)

func (k Keeper) TryUpdateCoinRates(ctx sdk.Context) {
	state := k.GetCoinRatesState(ctx)
	if state.Pending() {
		k.Logger(ctx).Info("Tried to update CoinRates but another request is pending")
		return
	}

	seq, err := k.sendCoinRatesRequest(ctx)
	if err != nil {
		// TODO: Implement a retry mechanism
		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeCoinRatesRequest,
				sdk.NewAttribute(types.AttributeError, err.Error()),
			))

		k.Logger(ctx).Error("Sending CoinRates request failed", "error", err)
		return
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeCoinRatesRequest,
			sdk.NewAttribute(types.AtributePacketSequence, fmt.Sprintf("%d", seq)),
		))
}

func (k Keeper) sendCoinRatesRequest(ctx sdk.Context) (uint64, error) {
	coinRatesParams := k.BandchainParams(ctx).CoinRatesParams

	callData := types.NewCoinRatesCallDataFromDecCoins(coinRatesParams.SymbolsWithMul)
	packetData := bandpacket.NewOracleRequestPacketData(
		types.CoinRatesClientID,
		coinRatesParams.ScriptParams.ScriptId,
		obi.MustEncode(callData),
		coinRatesParams.ScriptParams.AskCount,
		coinRatesParams.ScriptParams.MinCount,
		coinRatesParams.ScriptParams.FeeLimit,
		coinRatesParams.ScriptParams.PrepareGas,
		coinRatesParams.ScriptParams.ExecuteGas,
	)
	seq, err := k.sendOraclePacket(ctx, packetData)
	if err != nil {
		return 0, err
	}

	state := types.NewOracleScriptState(ctx, seq, &callData)
	k.setCoinRatesState(ctx, state)
	return seq, nil
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
	case types.CoinRatesClientID:
		var coinRatesResult types.CoinRatesResult
		if err := obi.Decode(packetData.GetResult(), &coinRatesResult); err != nil {
			return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "cannot decode the coinRates received packet")
		}

		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			if state.GetOracleRequestId() != packetData.GetRequestID() {
				return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "coinRates oracle request id %d does not match the state request id %d", packetData.GetRequestID(), state.GetOracleRequestId())
			}

			state.ResultPacketSequence = packet.GetSequence()
			state.SetResult(&coinRatesResult)
			return nil
		}); err != nil {
			return nil, err
		}

		k.updateStablePrices(ctx)
	default:
		return nil, sdkerrors.Wrapf(sdkerrors.ErrJSONUnmarshal, "oracle received packet not found: %s", packetData.GetClientID())
	}

	return types.ModuleCdc.MustMarshalJSON(
		bandpacket.NewOracleRequestPacketAcknowledgement(packetData.GetRequestID()),
	), nil
}

func coinRatesFromSymbols(symbols []string, rates []uint64) sdk.DecCoins {
	coins := make(sdk.DecCoins, len(symbols))
	for i, symbol := range symbols {
		coins[i] = sdk.NewDecCoinFromDec(symbol, sdk.NewDec(int64(rates[i])))
	}
	return coins
}

func (k Keeper) updateCoinRatesState(ctx sdk.Context, fn func(state *types.OracleScriptState) error) error {
	state := k.GetCoinRatesState(ctx)

	if err := fn(&state); err != nil {
		return err
	}
	state.UpdatedAtHeight = ctx.BlockHeight()

	k.setCoinRatesState(ctx, state)
	return nil
}

func (k Keeper) setCoinRatesState(ctx sdk.Context, state types.OracleScriptState) {
	store := ctx.KVStore(k.storeKey)
	store.Set(types.KeyCoinRatesState, k.cdc.MustMarshal(&state))
}

// GetCoinRatesState returns the coinRates state
func (k Keeper) GetCoinRatesState(ctx sdk.Context) types.OracleScriptState {
	store := ctx.KVStore(k.storeKey)
	var state types.OracleScriptState
	k.cdc.MustUnmarshal(store.Get(types.KeyCoinRatesState), &state)
	return state
}

func (k Keeper) handleOracleAcknowledgment(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	if !ack.Success() {
		err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			state.Failed = true
			return nil
		})
		if err != nil {
			return err
		}

		ctx.EventManager().EmitEvent(
			sdk.NewEvent(
				types.EventTypeOraclePacketAcknowledgement,
				sdk.NewAttribute(types.AttributeError, ack.GetError()),
			),
		)
		return nil
	}

	var ackData bandpacket.OracleRequestPacketAcknowledgement
	if err := types.ModuleCdc.UnmarshalJSON(ack.GetResult(), &ackData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet acknowledgement data")
	}

	var packetData bandpacket.OracleRequestPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientID:
		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			if state.GetRequestPacketSequence() != packet.GetSequence() {
				return sdkerrors.Wrapf(types.ErrInvalidPacketSequence, "expected packet sequence %d, got %d", state.GetRequestPacketSequence(), packet.GetSequence())
			}

			// Update request latest state with oracle request id
			state.OracleRequestId = ackData.GetRequestID()
			return nil
		}); err != nil {
			return err
		}
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "oracle acknowledgment handler for client id %s not found", packetData.GetClientID())
	}
}

func (k Keeper) handleOracleTimeout(ctx sdk.Context, packet channeltypes.Packet) error {
	var packetData bandpacket.OracleRequestPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientID:
		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			if state.GetRequestPacketSequence() != packet.GetSequence() {
				return sdkerrors.Wrapf(types.ErrInvalidPacketSequence, "expected packet sequence %d, got %d", state.GetRequestPacketSequence(), packet.GetSequence())
			}

			// Mark request as failed
			state.Failed = true
			return nil
		}); err != nil {
			return err
		}
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "oracle timeout handler for client id %s not found", packetData.GetClientID())
	}
}

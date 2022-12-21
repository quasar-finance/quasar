package keeper

import (
	"fmt"

	"github.com/bandprotocol/bandchain-packet/obi"
	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/utils"
)

func (k Keeper) TryUpdateCoinRates(ctx sdk.Context) {
	// Do not start a new procedure if module is disabled
	if !k.IsEnabled(ctx) {
		k.Logger(ctx).Info("module is disabled, skipping CoinRates update")
		return
	}

	// Do not start a new procedure if there's another one pending
	state := k.GetCoinRatesState(ctx)
	if state.Pending() {
		k.Logger(ctx).Info("tried to update CoinRates but another request is pending")
		return
	}

	_, err := k.sendCoinRatesRequest(ctx)
	if err != nil {
		k.Logger(ctx).Error("could not send CoinRates request to bandchain", "error", err)
		return
	}
}

func (k Keeper) sendCoinRatesRequest(ctx sdk.Context) (uint64, error) {
	coinRatesParams := k.GetCoinRatesParams(ctx)

	callData := types.NewCoinRatesCallData(coinRatesParams.Symbols)
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
	packet, err := utils.SendPacket(
		ctx,
		k.clientKeeper,
		k.ics4Wrapper,
		k.channelKeeper,
		k.scopedKeeper,
		k.GetPort(ctx),
		k.GetAuthorizedChannel(ctx),
		packetData.GetBytes(),
		k.GetPacketTimeoutHeight(ctx),
		k.GetPacketTimeoutTimestamp(ctx),
	)
	EmitOracleRequestEvent(ctx, &callData, packet, packetData, err)
	if err != nil {
		return 0, err
	}

	state := types.NewOracleScriptState(ctx, types.CoinRatesClientID, packet.GetSequence(), &callData)
	k.setCoinRatesState(ctx, state)

	return packet.GetSequence(), nil
}

func (k Keeper) OnRecvPacket(ctx sdk.Context, packet channeltypes.Packet) ([]byte, error) {
	var packetData bandpacket.OracleResponsePacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return nil, sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientID:
		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			if state.GetOracleRequestId() != packetData.GetRequestID() {
				k.Logger(ctx).Info(
					"received an oracle result inconsistent with current state",
					"packet_seq", fmt.Sprintf("%d", packet.GetSequence()),
					"packet_request_id", fmt.Sprintf("%d", packetData.GetRequestID()),
					"state_request_id", fmt.Sprintf("%d", state.GetOracleRequestId()),
				)
				return nil
			}

			state.ResultPacketSequence = packet.GetSequence()
			if packetData.ResolveStatus == bandpacket.RESOLVE_STATUS_SUCCESS {
				var coinRatesResult types.CoinRatesResult
				if err := obi.Decode(packetData.GetResult(), &coinRatesResult); err != nil {
					return sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "cannot decode the CoinRates result")
				}
				state.SetResult(&coinRatesResult)

				EmitOracleResponseEvent(ctx, &coinRatesResult, packet, packetData)
			} else {
				state.Fail()

				EmitOracleResponseEvent(ctx, nil, packet, packetData)
			}

			return nil
		}); err != nil {
			return nil, err
		}

		k.updatePriceList(ctx)
	default:
		return nil, sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "oracle received packet with unknown client id: %s", packetData.GetClientID())
	}

	return types.ModuleCdc.MustMarshalJSON(
		bandpacket.NewOracleRequestPacketAcknowledgement(packetData.GetRequestID()),
	), nil
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
	store.Set(types.CoinRatesStateKey, k.cdc.MustMarshal(&state))
}

// GetCoinRatesState returns the coinRates state
func (k Keeper) GetCoinRatesState(ctx sdk.Context) types.OracleScriptState {
	store := ctx.KVStore(k.storeKey)
	var state types.OracleScriptState
	k.cdc.MustUnmarshal(store.Get(types.CoinRatesStateKey), &state)
	return state
}

func (k Keeper) OnAcknowledgementPacket(ctx sdk.Context, packet channeltypes.Packet, ack channeltypes.Acknowledgement) error {
	var packetData bandpacket.OracleRequestPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientID:
		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			// Ignoring packets inconsistent with the current state
			if k.validatePacketAgainstState(ctx, packet, *state) {
				return nil
			}

			if !ack.Success() {
				state.Fail()

				ctx.EventManager().EmitEvent(
					sdk.NewEvent(
						types.EventTypeOraclePacketAcknowledgement,
						sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
						sdk.NewAttribute(types.AttributeKeyError, ack.GetError()),
					),
				)
			} else {
				var ackData bandpacket.OracleRequestPacketAcknowledgement
				if err := types.ModuleCdc.UnmarshalJSON(ack.GetResult(), &ackData); err != nil {
					return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet acknowledgement data")
				}
				// Update request latest state with oracle request id
				state.OracleRequestId = ackData.GetRequestID()

				ctx.EventManager().EmitEvent(
					sdk.NewEvent(
						types.EventTypeOraclePacketAcknowledgement,
						sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
						sdk.NewAttribute(types.AttributeKeyRequestId, fmt.Sprintf("%d", ackData.GetRequestID())),
					),
				)
			}

			return nil
		}); err != nil {
			return err
		}
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "oracle acknowledgment handler for client id %s not found", packetData.GetClientID())
	}
}

// validatePacketAgainstState returns true if the incoming ack/timeout is consistent with the given state
// else it returns false and it's callers responsibility to ignore the rest of process.
func (k Keeper) validatePacketAgainstState(ctx sdk.Context, packet channeltypes.Packet, state types.OracleScriptState) bool {
	if state.GetRequestPacketSequence() != packet.GetSequence() {
		k.Logger(ctx).Info(
			"received an oracle packet acknowledgement/timeout inconsistent with current state",
			"packet_seq", fmt.Sprintf("%d", packet.GetSequence()),
			"state_seq", fmt.Sprintf("%d", state.GetRequestPacketSequence()),
			"state_client_id", state.GetClientId(),
		)
		return true
	}
	return false
}

func (k Keeper) OnTimeoutPacket(ctx sdk.Context, packet channeltypes.Packet) error {
	var packetData bandpacket.OracleRequestPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &packetData); err != nil {
		return sdkerrors.Wrapf(err, "could not unmarshal bandchain oracle packet data")
	}

	switch packetData.GetClientID() {
	case types.CoinRatesClientID:
		if err := k.updateCoinRatesState(ctx, func(state *types.OracleScriptState) error {
			// Ignoring packets inconsistent with the current state
			if k.validatePacketAgainstState(ctx, packet, *state) {
				return nil
			}

			state.Fail()

			ctx.EventManager().EmitEvent(
				sdk.NewEvent(
					types.EventTypeOraclePacketAcknowledgement,
					sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
					sdk.NewAttribute(types.AttributeKeyError, channeltypes.ErrPacketTimeout.Error()),
				),
			)

			return nil
		}); err != nil {
			return err
		}
		return nil
	default:
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "oracle timeout handler for client id %s not found", packetData.GetClientID())
	}
}

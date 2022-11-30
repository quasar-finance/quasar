package transfer

import (
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	transfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	"github.com/quasarlabs/quasarnode/x/orion/types"
)

// HandleAcknowledgement passes the acknowledgement data to the onAckPacket function of the contract.
func (im IBCModule) HandleAcknowledgement(ctx sdk.Context, packet channeltypes.Packet, acknowledgement []byte,
	relayer sdk.AccAddress) error {
	var ack channeltypes.Acknowledgement
	if err := channeltypes.SubModuleCdc.UnmarshalJSON(acknowledgement, &ack); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet acknowledgement: %v", err)
	}
	var data transfertypes.FungibleTokenPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &data); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	contractAddr, err := sdk.AccAddressFromBech32(data.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}

	if ack.Success() {

		err := im.wasmKeeper.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
			Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
			OriginalPacket:  newIBCPacket(packet),
			Relayer:         relayer.String(),
		})

		if err != nil {
			im.keeper.Logger(ctx).Error("failed to re-enter contract on packet acknowledgement", err)
			return sdkerrors.Wrap(err, "failed to re-enter the contract on packet acknowledgement")
		}

	} else {
		// Actually we have only one kind of error returned from acknowledgement
		// maybe later we'll retrieve actual errors from events
		im.keeper.Logger(ctx).Error(ack.GetError(), "CheckTx", ctx.IsCheckTx())

		im.wasmKeeper.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
			Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
			OriginalPacket:  newIBCPacket(packet),
			Relayer:         relayer.String(),
		})
	}

	if err != nil {
		im.keeper.Logger(ctx).Error("failed to Sudo contract on packet acknowledgement", err)
		return sdkerrors.Wrap(err, "failed to Sudo the contract on packet acknowledgement")
	}

	im.keeper.Logger(ctx).Debug("acknowledgement received", "Packet data", data, "CheckTx", ctx.IsCheckTx())

	return nil
}

const portIDPrefix = ""

// todo: will someone please tell me how i can use this function without redefining it here, it exists in wasm: https://github.com/CosmWasm/wasmd/blob/a9ce273e3c1c4f7224e1293fcf1bfd5a50e4fe17/x/wasm/keeper/ibc.go#L40
func newIBCPacket(packet channeltypes.Packet) wasmvmtypes.IBCPacket {
	timeout := wasmvmtypes.IBCTimeout{
		Timestamp: packet.TimeoutTimestamp,
	}
	if !packet.TimeoutHeight.IsZero() {
		timeout.Block = &wasmvmtypes.IBCTimeoutBlock{
			Height:   packet.TimeoutHeight.RevisionHeight,
			Revision: packet.TimeoutHeight.RevisionNumber,
		}
	}

	return wasmvmtypes.IBCPacket{
		Data:     packet.Data,
		Src:      wasmvmtypes.IBCEndpoint{ChannelID: packet.SourceChannel, PortID: packet.SourcePort},
		Dest:     wasmvmtypes.IBCEndpoint{ChannelID: packet.DestinationChannel, PortID: packet.DestinationPort},
		Sequence: packet.Sequence,
		Timeout:  timeout,
	}
}

// HandleTimeout passes the timeout data to the appropriate contract via a Sudo call.
// Since all ICA channels are ORDERED, a single timeout shuts down a channel.
// The affected zone should be paused after a timeout.
func (im IBCModule) HandleTimeout(ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) error {
	var data transfertypes.FungibleTokenPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &data); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	contractAddr, err := sdk.AccAddressFromBech32(data.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}

	err2 := im.wasmKeeper.OnTimeoutPacket(ctx, contractAddr, wasmvmtypes.IBCPacketTimeoutMsg{Packet: newIBCPacket(packet), Relayer: relayer.String()})
	if err2 != nil {
		im.keeper.Logger(ctx).Error("failed to re-enter contract on packet timeout", err2)
		return sdkerrors.Wrap(err2, "failed to re-enter the contract on packet timeout")
	}

	return nil
}

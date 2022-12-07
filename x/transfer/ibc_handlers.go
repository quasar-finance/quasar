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

	contractInfo := im.wasmKeeper.GetContractInfo(ctx, contractAddr)
	if contractInfo.IBCPortID == "" {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "contract %s is not a valid IBC contract", contractAddr)
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

		err := im.wasmKeeper.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
			Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
			OriginalPacket:  newIBCPacket(packet),
			Relayer:         relayer.String(),
		})
		if err != nil {
			im.keeper.Logger(ctx).Error("failed to re-enter contract on packet timeout", err)
			return sdkerrors.Wrap(err, "failed to re-enter the contract on packet timeout")
		}
	}

	if err != nil {
		im.keeper.Logger(ctx).Error("failed to re-enter contract on packet acknowledgement", err)
		return sdkerrors.Wrap(err, "failed to re-enter the contract on packet acknowledgement")
	}

	im.keeper.Logger(ctx).Debug("acknowledgement received", "Packet data", data, "CheckTx", ctx.IsCheckTx())

	return nil
}

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

func (im IBCModule) HandleTimeout(ctx sdk.Context, packet channeltypes.Packet, relayer sdk.AccAddress) error {
	var data transfertypes.FungibleTokenPacketData
	if err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &data); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	contractAddr, err := sdk.AccAddressFromBech32(data.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}

	contractInfo := im.wasmKeeper.GetContractInfo(ctx, contractAddr)
	if contractInfo.IBCPortID == "" {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "contract %s is not a valid IBC contract", contractAddr)
	}

	err = im.wasmKeeper.OnTimeoutPacket(ctx, contractAddr, wasmvmtypes.IBCPacketTimeoutMsg{Packet: newIBCPacket(packet), Relayer: relayer.String()})
	if err != nil {
		im.keeper.Logger(ctx).Error("failed to re-enter contract on packet timeout", err)
		return sdkerrors.Wrap(err, "failed to re-enter the contract on packet timeout")
	}

	return nil
}

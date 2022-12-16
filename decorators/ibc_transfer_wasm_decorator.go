package decorators

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v5/modules/core/05-port/types"
	"github.com/tendermint/tendermint/libs/log"
)

var _ porttypes.IBCModule = IBCTransferWasmDecorator{}

// IBCTransferWasmDecorator is a decorator for ibc transfer module that will
// pass ack and timeout callbacks of wasm contracts that were the sender of packet to them.
// Note that the contracts should implement the IBC interface to receive the callbacks
// otherwise they won't receive any callbacks from this decorator and will be treated like
// a regular account.
// NOTICE: Potential Security Issue
type IBCTransferWasmDecorator struct {
	k *wasm.Keeper
	porttypes.IBCModule
}

// NewIBCTransferWasmDecorator returns a new IBCTransferWasmDecorator with the given wasm keeper and transfer ibc module.
func NewIBCTransferWasmDecorator(k *wasm.Keeper, m porttypes.IBCModule) IBCTransferWasmDecorator {
	return IBCTransferWasmDecorator{
		k:         k,
		IBCModule: m,
	}
}

// OnAcknowledgementPacket implements the IBCModule.OnAcknowledgementPacket
func (im IBCTransferWasmDecorator) OnAcknowledgementPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
) error {
	err := im.IBCModule.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := unmarshalTransferPacket(packet)
	if err != nil {
		return err
	}
	ack, err := unmarshalAcknowledgement(acknowledgement)
	if err != nil {
		return err
	}

	contractAddr, err := sdk.AccAddressFromBech32(transferPacket.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}
	contractInfo := im.k.GetContractInfo(ctx, contractAddr)
	// Skip if there's no contract with this address (it's a regular address) or the contract doesn't support IBC
	if contractInfo == nil || contractInfo.IBCPortID == "" {
		return nil
	}

	if !ack.Success() {
		im.logger(ctx).Debug(
			"passing an error acknowledgment to contract",
			"contract_address", contractAddr,
			"error", ack.GetError(),
		)
	}
	err = im.k.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
		Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
		OriginalPacket:  newWasmIBCPacket(packet),
		Relayer:         relayer.String(),
	})
	if err != nil {
		im.logger(ctx).Error(
			"contract returned error for acknowledgment",
			"contract_address", contractAddr,
			"error", err,
		)
		return sdkerrors.Wrap(err, "contract returned error for acknowledgment")
	}

	return nil
}

// OnTimeoutPacket implements the IBCModule.OnTimeoutPacket
func (im IBCTransferWasmDecorator) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	err := im.IBCModule.OnTimeoutPacket(ctx, packet, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := unmarshalTransferPacket(packet)
	if err != nil {
		return err
	}

	contractAddr, err := sdk.AccAddressFromBech32(transferPacket.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}
	contractInfo := im.k.GetContractInfo(ctx, contractAddr)
	if contractInfo.IBCPortID == "" {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "contract %s is not a valid IBC contract", contractAddr)
	}
	// Skip if there's no contract with this address (it's a regular address) or the contract doesn't support IBC
	if contractInfo == nil || contractInfo.IBCPortID == "" {
		return nil
	}

	err = im.k.OnTimeoutPacket(ctx, contractAddr, wasmvmtypes.IBCPacketTimeoutMsg{
		Packet:  newWasmIBCPacket(packet),
		Relayer: relayer.String(),
	})
	if err != nil {
		im.logger(ctx).Error(
			"contract returned error for timeout",
			"contract_address", contractAddr,
			"error", err,
		)
		return sdkerrors.Wrap(err, "contract returned error for timeout")
	}

	return nil
}

func newWasmIBCPacket(packet channeltypes.Packet) wasmvmtypes.IBCPacket {
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

func (im IBCTransferWasmDecorator) logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("decorator", "IBCTransferWasmCallback")
}

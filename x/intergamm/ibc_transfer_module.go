package intergamm

import (
	"fmt"

	"github.com/CosmWasm/wasmd/x/wasm"
	wasmvmtypes "github.com/CosmWasm/wasmvm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfer "github.com/cosmos/ibc-go/v3/modules/apps/transfer"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v3/modules/core/05-port/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/tendermint/tendermint/libs/log"
)

var _ porttypes.IBCModule = IBCTransferModuleDecorator{}

// IBCModule implements the ICS26 interface for interchain accounts controller chains
type IBCTransferModuleDecorator struct {
	k          *keeper.Keeper
	wasmKeeper *wasm.Keeper
	ibctransfer.IBCModule
}

// NewIBCModule creates a new IBCModule given the keeper
func NewIBCTransferModuleDecorator(k *keeper.Keeper, wk *wasm.Keeper, m ibctransfer.IBCModule) IBCTransferModuleDecorator {
	return IBCTransferModuleDecorator{
		k:          k,
		wasmKeeper: wk,
		IBCModule:  m,
	}
}

// OnChanOpenAck implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
	logger := im.k.Logger(ctx)
	logger.Info("OnChanOpenAck", "portID", portID,
		"channelID", channelID,
		"counterpartyChannelID", counterpartyChannelID,
		"counterpartyVersion", counterpartyVersion,
	)

	connectionID, _, err := im.k.GetChannelKeeper(ctx).GetChannelConnection(ctx, portID, channelID)
	if err != nil {
		return err
	}
	destinationChain, _ := im.k.GetChainID(ctx, connectionID)

	epi, found := im.k.GetPortDetail(ctx, destinationChain, portID)
	if found {
		// Don't update the im.k.SetPortDetail. As updating the new channel id will cause denom changes
		// to ibc token transfer. This make sure we use constant value of channel id for a given connection id/chain id
		logger.Info("OnChanOpenAck ibc-token-transfer ChannelID-PortID already exist", "PortInfo", epi)
		return im.IBCModule.OnChanOpenAck(ctx, portID, channelID, counterpartyChannelID, counterpartyVersion)
	}

	pi := types.PortInfo{
		PortID:                portID,
		ChannelID:             channelID,
		CounterpartyChannelID: counterpartyChannelID,
		ConnectionID:          connectionID,
	}
	logger.Info("OnChanOpenAck", "PortInfo", pi)

	im.k.SetPortDetail(ctx, pi)
	return im.IBCModule.OnChanOpenAck(ctx, portID, channelID, counterpartyChannelID, counterpartyVersion)
}

// OnAcknowledgementPacket implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnAcknowledgementPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
) error {
	var err error

	im.logger(ctx).Info("received OnAcknowledgementPacket", "seq", packet.GetSequence())

	err = im.IBCModule.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := parseTransferPacket(packet)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	var ack channeltypes.Acknowledgement
	err = types.ModuleCdc.UnmarshalJSON(acknowledgement, &ack)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet acknowledgement: %v", err)
	}

	err = im.sendAckToContract(
		ctx,
		packet,
		acknowledgement,
		relayer,
		&transferPacket,
		ack,
	)
	if err != nil {
		return sdkerrors.Wrap(err, "failed to send acknowledgment to contract")
	}

	return im.k.HandleIbcTransferAcknowledgement(ctx, packet.GetSequence(), transferPacket, ack)
}

func (im IBCTransferModuleDecorator) sendAckToContract(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
	packetData *ibctransfertypes.FungibleTokenPacketData,
	ack channeltypes.Acknowledgement,
) error {
	contractAddr, err := sdk.AccAddressFromBech32(packetData.GetSender())
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
			im.logger(ctx).Error("failed to re-enter contract on packet acknowledgement", err)
			return sdkerrors.Wrap(err, "failed to re-enter the contract on packet acknowledgement")
		}

	} else {
		// Actually we have only one kind of error returned from acknowledgement
		// maybe later we'll retrieve actual errors from events
		im.logger(ctx).Error(ack.GetError(), "CheckTx", ctx.IsCheckTx())

		err := im.wasmKeeper.OnAckPacket(ctx, contractAddr, wasmvmtypes.IBCPacketAckMsg{
			Acknowledgement: wasmvmtypes.IBCAcknowledgement{Data: acknowledgement},
			OriginalPacket:  newIBCPacket(packet),
			Relayer:         relayer.String(),
		})
		if err != nil {
			im.logger(ctx).Error("failed to re-enter contract on packet timeout", err)
			return sdkerrors.Wrap(err, "failed to re-enter the contract on packet timeout")
		}
	}

	if err != nil {
		im.logger(ctx).Error("failed to re-enter contract on packet acknowledgement", err)
		return sdkerrors.Wrap(err, "failed to re-enter the contract on packet acknowledgement")
	}

	im.logger(ctx).Debug("acknowledgement received", "Packet data", packetData, "CheckTx", ctx.IsCheckTx())

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

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCTransferModuleDecorator) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	var err error

	im.logger(ctx).Info("received OnTimeoutPacket", "seq", packet.GetSequence())

	err = im.IBCModule.OnTimeoutPacket(ctx, packet, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := parseTransferPacket(packet)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	err = im.sendTimeoutToContract(
		ctx,
		packet,
		relayer,
		&transferPacket,
	)
	if err != nil {
		return sdkerrors.Wrap(err, "failed to send acknowledgment to contract")
	}

	return im.k.HandleIbcTransferTimeout(ctx, packet.GetSequence(), transferPacket)
}

func (im IBCTransferModuleDecorator) sendTimeoutToContract(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
	packetData *ibctransfertypes.FungibleTokenPacketData,
) error {
	contractAddr, err := sdk.AccAddressFromBech32(packetData.GetSender())
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "failed to decode address from bech32: %v", err)
	}

	contractInfo := im.wasmKeeper.GetContractInfo(ctx, contractAddr)
	if contractInfo.IBCPortID == "" {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "contract %s is not a valid IBC contract", contractAddr)
	}

	err = im.wasmKeeper.OnTimeoutPacket(ctx, contractAddr, wasmvmtypes.IBCPacketTimeoutMsg{Packet: newIBCPacket(packet), Relayer: relayer.String()})
	if err != nil {
		im.logger(ctx).Error("failed to re-enter contract on packet timeout", err)
		return sdkerrors.Wrap(err, "failed to re-enter the contract on packet timeout")
	}

	return nil
}

func (im IBCTransferModuleDecorator) logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func parseTransferPacket(packet channeltypes.Packet) (ibctransfertypes.FungibleTokenPacketData, error) {
	var transferPacket ibctransfertypes.FungibleTokenPacketData
	err := types.ModuleCdc.UnmarshalJSON(packet.GetData(), &transferPacket)
	if err != nil {
		return transferPacket, sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	return transferPacket, nil
}

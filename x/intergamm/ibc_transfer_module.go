package intergamm

import (
	"fmt"

	"github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v3/modules/core/05-port/types"
	ibcexported "github.com/cosmos/ibc-go/v3/modules/core/exported"
	"github.com/tendermint/tendermint/libs/log"

	ibctransfer "github.com/cosmos/ibc-go/v3/modules/apps/transfer"
)

var _ porttypes.IBCModule = IBCModule{}

// IBCModule implements the ICS26 interface for interchain accounts controller chains
type IBCTransferModuleDecorator struct {
	m *ibctransfer.IBCModule
	k *keeper.Keeper
}

// NewIBCModule creates a new IBCModule given the keeper
func NewIBCTransferModuleDecorator(m *ibctransfer.IBCModule, k *keeper.Keeper) IBCTransferModuleDecorator {
	return IBCTransferModuleDecorator{
		m: m,
		k: k,
	}
}

// OnChanOpenInit implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanOpenInit(
	ctx sdk.Context,
	order channeltypes.Order,
	connectionHops []string,
	portID string,
	channelID string,
	chanCap *capabilitytypes.Capability,
	counterparty channeltypes.Counterparty,
	version string,
) error {
	return im.m.OnChanOpenInit(ctx, order, connectionHops, portID, channelID, chanCap, counterparty, version)
}

// OnChanOpenTry implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanOpenTry(
	ctx sdk.Context,
	order channeltypes.Order,
	connectionHops []string,
	portID,
	channelID string,
	chanCap *capabilitytypes.Capability,
	counterparty channeltypes.Counterparty,
	counterpartyVersion string,
) (string, error) {
	return im.m.OnChanOpenTry(ctx, order, connectionHops, portID, channelID, chanCap, counterparty, counterpartyVersion)
}

// OnChanOpenAck implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
	return im.m.OnChanOpenAck(ctx, portID, channelID, counterpartyChannelID, counterpartyVersion)
}

// OnChanOpenConfirm implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanOpenConfirm(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return im.m.OnChanOpenConfirm(ctx, portID, channelID)
}

// OnChanCloseInit implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanCloseInit(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return im.m.OnChanCloseInit(ctx, portID, channelID)
}

// OnChanCloseConfirm implements the IBCModule interface
func (im IBCTransferModuleDecorator) OnChanCloseConfirm(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return im.m.OnChanCloseConfirm(ctx, portID, channelID)
}

// OnRecvPacket implements the IBCModule interface. A successful acknowledgement
// is returned if the packet data is succesfully decoded and the receive application
// logic returns without error.
func (im IBCTransferModuleDecorator) OnRecvPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) ibcexported.Acknowledgement {
	return im.m.OnRecvPacket(ctx, packet, relayer)
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

	err = im.m.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
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

	return im.k.HandleIbcTransferAcknowledgement(ctx, packet.GetSequence(), transferPacket, ack)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCTransferModuleDecorator) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	var err error

	im.logger(ctx).Info("received OnTimeoutPacket", "seq", packet.GetSequence())

	err = im.m.OnTimeoutPacket(ctx, packet, relayer)
	if err != nil {
		return err
	}

	transferPacket, err := parseTransferPacket(packet)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	return im.k.HandleIbcTransferTimeout(ctx, packet.GetSequence(), transferPacket)
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

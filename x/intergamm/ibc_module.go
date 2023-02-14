package intergamm

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	icatypes "github.com/cosmos/ibc-go/v5/modules/apps/27-interchain-accounts/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v5/modules/core/05-port/types"
	host "github.com/cosmos/ibc-go/v5/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v5/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/tendermint/tendermint/libs/log"
)

var _ porttypes.IBCModule = IBCModule{}

// IBCModule implements the ICS26 interface for interchain accounts controller chains
type IBCModule struct {
	keeper *keeper.Keeper
}

// NewIBCModule creates a new IBCModule given the keeper
func NewIBCModule(k *keeper.Keeper) IBCModule {
	return IBCModule{
		keeper: k,
	}
}

// OnChanOpenInit implements the IBCModule interface
func (im IBCModule) OnChanOpenInit(
	ctx sdk.Context,
	order channeltypes.Order,
	connectionHops []string,
	portID string,
	channelID string,
	chanCap *capabilitytypes.Capability,
	counterparty channeltypes.Counterparty,
	version string,
) (string, error) {
	if err := im.keeper.ClaimCapability(ctx, chanCap, host.ChannelCapabilityPath(portID, channelID)); err != nil {
		return "", err
	}

	return version, nil
}

// OnChanOpenTry implements the IBCModule interface
func (im IBCModule) OnChanOpenTry(
	ctx sdk.Context,
	order channeltypes.Order,
	connectionHops []string,
	portID,
	channelID string,
	chanCap *capabilitytypes.Capability,
	counterparty channeltypes.Counterparty,
	counterpartyVersion string,
) (string, error) {
	return "", nil
}

// OnChanOpenAck implements the IBCModule interface
func (im IBCModule) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
	logger := im.keeper.Logger(ctx)
	logger.Info("OnChanOpenAck ICS26", "portID", portID,
		"channelID", channelID,
		"counterpartyChannelID", counterpartyChannelID,
		"counterpartyVersion", counterpartyVersion,
	)
	connectionID, _, err := im.keeper.GetChannelKeeper(ctx).GetChannelConnection(ctx, portID, channelID)
	if err != nil {
		return err
	}
	pi := types.PortInfo{
		PortID:                portID,
		ChannelID:             channelID,
		CounterpartyChannelID: counterpartyChannelID,
		ConnectionID:          connectionID,
	}

	logger.Info("OnChanOpenAck ICS26", "PortInfo", pi)
	im.keeper.SetPortDetail(ctx, pi)
	return nil
}

// OnChanOpenConfirm implements the IBCModule interface
func (im IBCModule) OnChanOpenConfirm(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return nil
}

// OnChanCloseInit implements the IBCModule interface
func (im IBCModule) OnChanCloseInit(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return nil
}

// OnChanCloseConfirm implements the IBCModule interface
func (im IBCModule) OnChanCloseConfirm(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	return nil
}

// OnRecvPacket implements the IBCModule interface. A successful acknowledgement
// is returned if the packet data is successfully decoded and the receive application
// logic returns without error.
func (im IBCModule) OnRecvPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) ibcexported.Acknowledgement {
	err := sdkerrors.Wrapf(icatypes.ErrInvalidChannelFlow, "cannot receive packet via interchain accounts authentication module")
	return channeltypes.NewErrorAcknowledgement(err)
}

// OnAcknowledgementPacket implements the IBCModule interface
func (im IBCModule) OnAcknowledgementPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
) error {
	var err error

	im.logger(ctx).Info("received OnAcknowledgementPacket", "seq", packet.GetSequence())

	icaPacket, err := parseIcaPacket(packet)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-27 ica packet data: %s", err.Error())
	}

	ack := channeltypes.Acknowledgement{}
	err = icatypes.ModuleCdc.UnmarshalJSON(acknowledgement, &ack)
	if err != nil {
		return sdkerrors.Wrapf(icatypes.ErrUnknownDataType, "cannot unmarshal IBC acknowledgement")
	}

	return im.keeper.HandleIcaAcknowledgement(ctx, packet.GetSequence(), packet.SourceChannel, packet.SourcePort, icaPacket, ack)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCModule) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	im.logger(ctx).Info("received OnTimeoutPacket", "seq", packet.GetSequence())

	icaPacket, err := parseIcaPacket(packet)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-27 ica packet data: %s", err.Error())
	}

	return im.keeper.HandleIcaTimeout(ctx, packet.GetSequence(), icaPacket)
}

// NegotiateAppVersion implements the IBCModule interface
func (im IBCModule) NegotiateAppVersion(
	ctx sdk.Context,
	order channeltypes.Order,
	connectionID string,
	portID string,
	counterparty channeltypes.Counterparty,
	proposedVersion string,
) (string, error) {
	return "", nil
}

func (im IBCModule) logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("module", fmt.Sprintf("x/%s", types.ModuleName))
}

func parseIcaPacket(packet channeltypes.Packet) (icatypes.InterchainAccountPacketData, error) {
	var icaPacket icatypes.InterchainAccountPacketData
	err := icatypes.ModuleCdc.UnmarshalJSON(packet.GetData(), &icaPacket)
	if err != nil {
		return icaPacket, sdkerrors.Wrapf(icatypes.ErrUnknownDataType, "cannot unmarshal ICS-27 interchain account packet data")
	}

	if icaPacket.Type != icatypes.EXECUTE_TX {
		return icaPacket, sdkerrors.Wrapf(icatypes.ErrUnsupported, "only EXECUTE_TX ICA callbacks are supported")
	}

	return icaPacket, nil
}

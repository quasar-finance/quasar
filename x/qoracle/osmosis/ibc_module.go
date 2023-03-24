package qoracle

import (
	"strings"

	// "cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	host "github.com/cosmos/ibc-go/v4/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v4/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	icqtypes "github.com/strangelove-ventures/async-icq/v4/types"
)

var _ porttypes.IBCModule = IBCModule{}

// IBCModule implements the ICS26 interface for osmosis qoracle sub module given the keeper
type IBCModule struct {
	keeper keeper.Keeper
}

// NewIBCModule creates a new IBCModule given the keeper
func NewIBCModule(k keeper.Keeper) IBCModule {
	return IBCModule{
		keeper: k,
	}
}

// OnChanOpenInit implements the IBCModule interface
func (im IBCModule) OnChanOpenInit(
	ctx sdk.Context,
	order channeltypes.Order,
	_ []string,
	portID string,
	channelID string,
	chanCap *capabilitytypes.Capability,
	_ channeltypes.Counterparty,
	version string,
) (string, error) {
	if !im.keeper.IsEnabled(ctx) {
		return "", types.ErrDisabled
	}

	if err := im.validateChannelParams(ctx, order, portID); err != nil {
		return "", err
	}

	if strings.TrimSpace(version) == "" {
		version = icqtypes.Version
	}

	if version != icqtypes.Version {
		return "", sdkerrors.Wrapf(channeltypes.ErrInvalidChannelVersion, "got %s, expected %s", version, icqtypes.Version)
	}

	// Claim channel capability passed back by IBC module
	if err := im.keeper.ClaimCapability(ctx, chanCap, host.ChannelCapabilityPath(portID, channelID)); err != nil {
		return "", err
	}

	return version, nil
}

func (im IBCModule) validateChannelParams(
	ctx sdk.Context,
	order channeltypes.Order,
	portID string,
) error {
	if order != channeltypes.UNORDERED {
		return sdkerrors.Wrapf(channeltypes.ErrInvalidChannelOrdering, "expected %s channel, got %s ", channeltypes.UNORDERED, order)
	}

	// Require port id to be the port id module is bound to
	boundPort := im.keeper.GetPort(ctx)
	if boundPort != portID {
		return sdkerrors.Wrapf(porttypes.ErrInvalidPort, "invalid port: %s, expected %s", portID, boundPort)
	}

	return nil
}

// OnChanOpenTry implements the IBCModule interface
func (im IBCModule) OnChanOpenTry(
	ctx sdk.Context,
	order channeltypes.Order,
	_ []string,
	portID,
	channelID string,
	chanCap *capabilitytypes.Capability,
	_ channeltypes.Counterparty,
	counterpartyVersion string,
) (string, error) {
	if !im.keeper.IsEnabled(ctx) {
		return "", types.ErrDisabled
	}

	if err := im.validateChannelParams(ctx, order, portID); err != nil {
		return "", err
	}

	if counterpartyVersion != icqtypes.Version {
		return "", sdkerrors.Wrapf(channeltypes.ErrInvalidChannelVersion, "invalid counterparty version: %s, expected %s", counterpartyVersion, icqtypes.Version)
	}

	// OpenTry must claim the channelCapability that IBC passes into the callback
	if err := im.keeper.ClaimCapability(ctx, chanCap, host.ChannelCapabilityPath(portID, channelID)); err != nil {
		return "", err
	}

	return icqtypes.Version, nil
}

// OnChanOpenAck implements the IBCModule interface
func (IBCModule) OnChanOpenAck(
	_ sdk.Context,
	_ string,
	_ string,
	_ string,
	counterpartyVersion string,
) error {
	if counterpartyVersion != icqtypes.Version {
		return sdkerrors.Wrapf(channeltypes.ErrInvalidChannelVersion, "invalid counterparty version: %s, expected %s", counterpartyVersion, icqtypes.Version)
	}
	return nil
}

// OnChanOpenConfirm implements the IBCModule interface
func (im IBCModule) OnChanOpenConfirm(
	ctx sdk.Context,
	_ string,
	_ string,
) error {
	if !im.keeper.IsEnabled(ctx) {
		return types.ErrDisabled
	}

	return nil
}

// OnChanCloseInit implements the IBCModule interface
func (IBCModule) OnChanCloseInit(
	_ sdk.Context,
	_ string,
	_ string,
) error {
	return nil
}

// OnChanCloseConfirm implements the IBCModule interface
func (IBCModule) OnChanCloseConfirm(
	_ sdk.Context,
	_ string,
	_ string,
) error {
	return nil
}

// OnRecvPacket implements the IBCModule interface.
func (IBCModule) OnRecvPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	_ sdk.AccAddress,
) ibcexported.Acknowledgement {
	err := sdkerrors.Wrapf(types.ErrInvalidChannelFlow, "cannot receive packet on qoracle module")
	ack := channeltypes.NewErrorAcknowledgement(err)
	keeper.EmitAcknowledgementEvent(ctx, packet, ack, err)
	return ack
}

// OnAcknowledgementPacket implements the IBCModule interface
func (im IBCModule) OnAcknowledgementPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
) error {
	var ack channeltypes.Acknowledgement
	if err := types.ModuleCdc.UnmarshalJSON(acknowledgement, &ack); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest,
			"cannot unmarshal ibc packet acknowledgement: %v, relayer: %s", err, relayer.String())
	}

	return im.keeper.OnAcknowledgementPacket(ctx, packet, ack)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCModule) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	im.keeper.Logger(ctx).Error("osmosis param request state is timed out.",
		"relayer address", relayer.String())
	return im.keeper.OnTimeoutPacket(ctx, packet)
}

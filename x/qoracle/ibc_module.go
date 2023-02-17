package qoracle

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	// icqtypes "github.com/cosmos/ibc-go/v4/modules/apps/icq/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	host "github.com/cosmos/ibc-go/v4/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v4/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	icqtypes "github.com/strangelove-ventures/async-icq/v4/types"
)

var _ porttypes.IBCModule = IBCModule{}

// IBCModule implements the ICS26 interface for interchain accounts controller chains
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
	connectionHops []string,
	portID string,
	channelID string,
	chanCap *capabilitytypes.Capability,
	counterparty channeltypes.Counterparty,
	version string,
) (string, error) {
	if err := im.validateChannelParams(ctx, order, portID, counterparty, version); err != nil {
		return version, err
	}

	// Note: this is only for testing purposes, and should be removed in the future
	// Setting the AuthorizedChannel based on counterparty port id
	switch counterparty.GetPortID() {
	case types.BandchainOraclePortID:
		bandchainParams := im.keeper.BandchainParams(ctx)
		bandchainParams.OracleIbcParams.AuthorizedChannel = channelID
		im.keeper.SetBandchainParams(ctx, bandchainParams)

		im.keeper.Logger(ctx).Info("Bandchain authorized channel set to: ", channelID)
	case icqtypes.PortID:
		osmosisParams := im.keeper.OsmosisParams(ctx)
		osmosisParams.ICQParams.AuthorizedChannel = channelID
		im.keeper.SetOsmosisParams(ctx, osmosisParams)

		im.keeper.Logger(ctx).Info("Osmosis ICQ authorized channel set to: ", channelID)
	}

	return version, im.keeper.ClaimCapability(ctx, chanCap, host.ChannelCapabilityPath(portID, channelID))
}

func (im IBCModule) validateChannelParams(
	ctx sdk.Context,
	order channeltypes.Order,
	portID string,
	counterparty channeltypes.Counterparty,
	version string,
) error {
	if order != channeltypes.UNORDERED {
		return sdkerrors.Wrapf(channeltypes.ErrInvalidChannelOrdering, "expected %s channel, got %s ", channeltypes.UNORDERED, order)
	}

	// Require port id to be the port id qoracle module is bound to
	boundPort := im.keeper.GetPort(ctx)
	if boundPort != portID {
		return sdkerrors.Wrapf(porttypes.ErrInvalidPort, "invalid port: %s, expected %s", portID, boundPort)
	}

	switch counterparty.GetPortID() {
	case types.BandchainOraclePortID:
		if version != types.BandchainOracleVersion {
			return sdkerrors.Wrapf(types.ErrInvalidCounterpartyVersion, "got %s, expected %s", version, types.BandchainOracleVersion)
		}
	case icqtypes.PortID:
		if version != icqtypes.Version {
			return sdkerrors.Wrapf(types.ErrInvalidCounterpartyVersion, "got %s, expected %s", version, icqtypes.Version)
		}
	default:
		return sdkerrors.Wrapf(porttypes.ErrInvalidPort, "invalid counterparty port: %s", counterparty.GetPortID())
	}

	return nil
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
	return "", sdkerrors.Wrap(types.ErrInvalidChannelFlow, "channel handshake must be initiated by controller chain")
}

// OnChanOpenAck implements the IBCModule interface
func (im IBCModule) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
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
	// Disallow user-initiated channel closing for any channels
	return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "user cannot close channel")
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
// is returned if the packet data is succesfully decoded and the receive application
// logic returns without error.
func (im IBCModule) OnRecvPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) ibcexported.Acknowledgement {
	resp, err := im.keeper.OnRecvPacket(ctx, packet)
	if err != nil {
		return types.NewErrorAcknowledgement(err)
	}

	return channeltypes.NewResultAcknowledgement(resp)
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
		return sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal packet acknowledgement: %v", err)
	}

	return im.keeper.OnAcknowledgementPacket(ctx, packet, ack)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCModule) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	return im.keeper.OnTimeoutPacket(ctx, packet)
}

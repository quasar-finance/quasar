package qoracle

import (
	"strings"

	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v5/modules/core/05-port/types"
	host "github.com/cosmos/ibc-go/v5/modules/core/24-host"
	ibcexported "github.com/cosmos/ibc-go/v5/modules/core/exported"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/keeper"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
)

var _ porttypes.IBCModule = IBCModule{}

// IBCModule implements the ICS26 interface for bandchain qoracle sub module given the keeper
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
	if !im.keeper.IsEnabled(ctx) {
		return "", types.ErrDisabled
	}

	if err := im.validateChannelParams(ctx, order, portID); err != nil {
		return "", err
	}

	if strings.TrimSpace(version) == "" {
		version = types.BandchainOracleVersion
	}

	if version != types.BandchainOracleVersion {
		return "", errors.Wrapf(channeltypes.ErrInvalidChannelVersion, "got %s, expected %s", version, types.BandchainOracleVersion)
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
		return errors.Wrapf(channeltypes.ErrInvalidChannelOrdering, "expected %s channel, got %s ", channeltypes.UNORDERED, order)
	}

	// Require port id to be the port id module is bound to
	boundPort := im.keeper.GetPort(ctx)
	if boundPort != portID {
		return errors.Wrapf(porttypes.ErrInvalidPort, "invalid port: %s, expected %s", portID, boundPort)
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
	if !im.keeper.IsEnabled(ctx) {
		return "", types.ErrDisabled
	}

	if err := im.validateChannelParams(ctx, order, portID); err != nil {
		return "", err
	}

	if counterpartyVersion != types.BandchainOracleVersion {
		return "", errors.Wrapf(channeltypes.ErrInvalidChannelVersion, "invalid counterparty version: %s, expected %s", counterpartyVersion, types.BandchainOracleVersion)
	}

	// OpenTry must claim the channelCapability that IBC passes into the callback
	if err := im.keeper.ClaimCapability(ctx, chanCap, host.ChannelCapabilityPath(portID, channelID)); err != nil {
		return "", err
	}

	return types.BandchainOracleVersion, nil
}

// OnChanOpenAck implements the IBCModule interface
func (im IBCModule) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
	if counterpartyVersion != types.BandchainOracleVersion {
		return errors.Wrapf(channeltypes.ErrInvalidChannelVersion, "invalid counterparty version: %s, expected %s", counterpartyVersion, types.BandchainOracleVersion)
	}
	return nil
}

// OnChanOpenConfirm implements the IBCModule interface
func (im IBCModule) OnChanOpenConfirm(
	ctx sdk.Context,
	portID,
	channelID string,
) error {
	if !im.keeper.IsEnabled(ctx) {
		return types.ErrDisabled
	}

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
// is returned if the packet data is succesfully decoded and the receive application
// logic returns without error.
func (im IBCModule) OnRecvPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) ibcexported.Acknowledgement {
	if !im.keeper.IsEnabled(ctx) {
		return channeltypes.NewErrorAcknowledgement(types.ErrDisabled)
	}

	txResponse, err := im.keeper.OnRecvPacket(ctx, packet)
	ack := channeltypes.NewResultAcknowledgement(txResponse)
	if err != nil {
		ack = channeltypes.NewErrorAcknowledgement(err)
	}

	// NOTE: acknowledgement will be written synchronously during IBC handler execution.
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
		return errors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal packet acknowledgement: %v", err)
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

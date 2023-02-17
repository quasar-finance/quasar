package decorators

/*
import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibctransfertypes "github.com/cosmos/ibc-go/v4/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	porttypes "github.com/cosmos/ibc-go/v4/modules/core/05-port/types"
	intergammkeeper "github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
	"github.com/tendermint/tendermint/libs/log"
)

var _ porttypes.IBCModule = IBCTransferIntergammDecorator{}

// IBCModule implements the ICS26 interface for interchain accounts controller chains
type IBCTransferIntergammDecorator struct {
	k *intergammkeeper.Keeper
	porttypes.IBCModule
}

// NewIBCModule creates a new IBCModule given the intergamm keeper
func NewIBCTransferIntergammDecorator(k *intergammkeeper.Keeper, m porttypes.IBCModule) IBCTransferIntergammDecorator {
	return IBCTransferIntergammDecorator{
		k:         k,
		IBCModule: m,
	}
}

// OnChanOpenAck implements the IBCModule.OnChanOpenAck
func (im IBCTransferIntergammDecorator) OnChanOpenAck(
	ctx sdk.Context,
	portID,
	channelID string,
	counterpartyChannelID string,
	counterpartyVersion string,
) error {
	connectionID, _, err := im.k.GetChannelKeeper(ctx).GetChannelConnection(ctx, portID, channelID)
	if err != nil {
		return err
	}
	destinationChain, _ := im.k.GetChainID(ctx, connectionID)

	_, found := im.k.GetPortDetail(ctx, destinationChain, portID)
	// Don't update the im.k.SetPortDetail. As updating the new channel id will cause denom changes
	// to ibc token transfer. This make sure we use constant value of channel id for a given connection id/chain id
	if !found {
		pi := intergammtypes.PortInfo{
			PortID:                portID,
			ChannelID:             channelID,
			CounterpartyChannelID: counterpartyChannelID,
			ConnectionID:          connectionID,
		}
		im.k.SetPortDetail(ctx, pi)
		im.logger(ctx).Info(
			"created new port detail",
			"port_detail", pi,
		)
	}

	return im.IBCModule.OnChanOpenAck(ctx, portID, channelID, counterpartyChannelID, counterpartyVersion)
}

// OnAcknowledgementPacket implements the IBCModule.OnAcknowledgementPacket
func (im IBCTransferIntergammDecorator) OnAcknowledgementPacket(
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

	return im.k.HandleIbcTransferAcknowledgement(ctx, packet.GetSequence(), transferPacket, ack)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCTransferIntergammDecorator) OnTimeoutPacket(
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

	return im.k.HandleIbcTransferTimeout(ctx, packet.GetSequence(), transferPacket)
}

func (im IBCTransferIntergammDecorator) logger(ctx sdk.Context) log.Logger {
	return ctx.Logger().With("decorator", "IBCTransferIntergammDecorator")
}

func unmarshalTransferPacket(packet channeltypes.Packet) (ibctransfertypes.FungibleTokenPacketData, error) {
	var transferPacket ibctransfertypes.FungibleTokenPacketData
	err := intergammtypes.ModuleCdc.UnmarshalJSON(packet.GetData(), &transferPacket)
	if err != nil {
		return transferPacket, sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet data: %s", err.Error())
	}

	return transferPacket, nil
}

func unmarshalAcknowledgement(acknowledgement []byte) (channeltypes.Acknowledgement, error) {
	var ack channeltypes.Acknowledgement
	err := intergammtypes.ModuleCdc.UnmarshalJSON(acknowledgement, &ack)
	if err != nil {
		return ack, sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "cannot unmarshal ICS-20 transfer packet acknowledgement: %v", err)
	}
	return ack, nil
}
*/

package keeper

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

// EmitOsmosisRequestEvent emits an event signalling a successful or failed icq request to fetch osmosis chain params and including the error
// details if there's any.
func EmitOsmosisRequestEvent(
	ctx sdk.Context,
	title string,
	packet channeltypes.Packet,
	err error,
) {
	attributes := []sdk.Attribute{
		sdk.NewAttribute(sdk.AttributeKeyModule, types.SubModuleName),
		sdk.NewAttribute(types.AttributeKeyPacketChannelId, packet.GetSourceChannel()),
		sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
		sdk.NewAttribute(types.AttributeKeyTitle, title),
	}
	if err != nil {
		attributes = append(attributes, sdk.NewAttribute(types.AttributeKeyError, err.Error()))
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeOsmosisRequest,
			attributes...,
		),
	)
}

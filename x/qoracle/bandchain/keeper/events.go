package keeper

import (
	"fmt"

	bandpacket "github.com/bandprotocol/bandchain-packet/packet"
	sdk "github.com/cosmos/cosmos-sdk/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
)

// EmitOracleRequestEvent emits an event signalling a successful or failed bandchain oracle request and including the error
// details if there's any.
func EmitOracleRequestEvent(
	ctx sdk.Context,
	callData fmt.Stringer,
	packet channeltypes.Packet,
	packetData bandpacket.OracleRequestPacketData,
	err error,
) {
	attributes := []sdk.Attribute{
		sdk.NewAttribute(sdk.AttributeKeyModule, types.SubModuleName),
		sdk.NewAttribute(types.AttributeKeyPacketChannelId, packet.GetSourceChannel()),
		sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
		sdk.NewAttribute(types.AttributeKeyClientID, packetData.GetClientID()),
		sdk.NewAttribute(types.AttributeKeyScriptID, fmt.Sprintf("%d", packetData.GetOracleScriptID())),
		sdk.NewAttribute(types.AttributeKeyCallData, callData.String()),
		sdk.NewAttribute(types.AttributeKeyAskCount, fmt.Sprintf("%d", packetData.GetAskCount())),
		sdk.NewAttribute(types.AttributeKeyMinCount, fmt.Sprintf("%d", packetData.GetMinCount())),
		sdk.NewAttribute(types.AttributeKeyFeeLimit, packetData.GetFeeLimit().String()),
		sdk.NewAttribute(types.AttributeKeyPrepareGas, fmt.Sprintf("%d", packetData.GetPrepareGas())),
		sdk.NewAttribute(types.AttributeKeyExecuteGas, fmt.Sprintf("%d", packetData.GetExecuteGas())),
	}
	if err != nil {
		attributes = append(attributes, sdk.NewAttribute(types.AttributeKeyError, err.Error()))
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeCoinRatesRequest,
			attributes...,
		),
	)
}

// EmitOracleResponseEvent emits an event signalling a successful or failed bandchain oracle response and including the error
// details if there's any.
func EmitOracleResponseEvent(
	ctx sdk.Context,
	result fmt.Stringer,
	packet channeltypes.Packet,
	packetData bandpacket.OracleResponsePacketData,
) {
	attributes := []sdk.Attribute{
		sdk.NewAttribute(sdk.AttributeKeyModule, types.SubModuleName),
		sdk.NewAttribute(types.AttributeKeyPacketChannelId, packet.GetDestChannel()),
		sdk.NewAttribute(types.AttributeKeyPacketSequence, fmt.Sprintf("%d", packet.GetSequence())),
		sdk.NewAttribute(types.AttributeKeyClientID, packetData.GetClientID()),
		sdk.NewAttribute(types.AttributeKeyRequestId, fmt.Sprintf("%d", packetData.GetRequestID())),
		sdk.NewAttribute(types.AttributeKeyAnsCount, fmt.Sprintf("%d", packetData.GetAnsCount())),
		sdk.NewAttribute(types.AttributeKeyRequestTime, fmt.Sprintf("%d", packetData.GetRequestTime())),
		sdk.NewAttribute(types.AttributeKeyResolveTime, fmt.Sprintf("%d", packetData.GetResolveTime())),
		sdk.NewAttribute(types.AttributeKeyResolveStatus, packetData.GetResolveStatus().String()),
	}
	if result != nil {
		attributes = append(attributes, sdk.NewAttribute(types.AttributeKeyResult, result.String()))
	}

	ctx.EventManager().EmitEvent(
		sdk.NewEvent(
			types.EventTypeCoinRatesRequest,
			attributes...,
		),
	)
}

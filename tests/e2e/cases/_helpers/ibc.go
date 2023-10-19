package helpers

import (
	transfertypes "github.com/cosmos/ibc-go/v7/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"
)

// IbcDenomFromChannel returns ibc denom according to the given channel port, id and denom
// this function generates the ibc denom for the main direction as an example if there is a channel from
// chain1 <-> chain2 knowing that chain1 has a denom named denom1 this function will return the ibc denom of denom1 in chain2.
func IbcDenomFromChannel(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.PortId, ch.ChannelId, baseDenom)).IBCDenom()
}

// IbcDenomFromChannelCounterparty does same as ibcDenomFromChannel but in reverse, so it generates
// the ibc denom of denom2 from chain2 (counterparty chain) in chain1
func IbcDenomFromChannelCounterparty(ch *channeltypes.IdentifiedChannel, baseDenom string) string {
	return transfertypes.ParseDenomTrace(transfertypes.GetPrefixedDenom(ch.Counterparty.PortId, ch.Counterparty.ChannelId, baseDenom)).IBCDenom()
}

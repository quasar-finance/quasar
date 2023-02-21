package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	channeltypes "github.com/cosmos/ibc-go/v4/modules/core/04-channel/types"
	ibcexported "github.com/cosmos/ibc-go/v4/modules/core/exported"
)

// ChannelKeeper defines the expected IBC channel keeper
type ChannelKeeper interface {
	GetChannel(ctx sdk.Context, srcPort, srcChan string) (channel channeltypes.Channel, found bool)
	GetNextSequenceSend(ctx sdk.Context, portID, channelID string) (uint64, bool)
	GetConnection(ctx sdk.Context, connectionID string) (ibcexported.ConnectionI, error)
	GetChannelClientState(ctx sdk.Context, portID, channelID string) (string, ibcexported.ClientState, error)
	GetChannelConnection(ctx sdk.Context, portID, channelID string) (string, ibcexported.ConnectionI, error)
}

// PortKeeper defines the expected IBC port keeper
type PortKeeper interface {
	BindPort(ctx sdk.Context, portID string) *capabilitytypes.Capability
	IsBound(ctx sdk.Context, portID string) bool
}

// ClientKeeper defines the expected IBC client keeper
type ClientKeeper interface {
	GetClientConsensusState(ctx sdk.Context, clientID string, height ibcexported.Height) (ibcexported.ConsensusState, bool)
}

// PriceOracle defines an interface for price oracle submodules that will
// fetch price of pre-defined list of symbols from oracle sources and deliver
// them to qoracle module when needed.
type PriceOracle interface {
	Oracle
	GetSymbolPriceList(ctx sdk.Context) (SymbolPriceList, error)
}

// PoolOracle defines an interface for pool oracle submodules that will
// fetch pools from chains like osmosis and etc and deliver them to qoracle
// with calculated TVL and APY.
type PoolOracle interface {
	Oracle
	GetPools(ctx sdk.Context) ([]Pool, error)
}

// Oracle defines an interface for oracle submodules.
type Oracle interface {
	// Source returns the name of the oracle source. Note that the name must be unique.
	Source() string
}

package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qoracle module sentinel errors
var (
	ErrSample                       = sdkerrors.Register(ModuleName, 1100, "sample error")
	ErrInvalidStablePrice           = sdkerrors.Register(ModuleName, 200, "invalid stable price")
	ErrUnAuthorizedOracleClient     = sdkerrors.Register(ModuleName, 201, "unauthorized oracle client")
	ErrInvalidCounterpartyVersion   = sdkerrors.Register(ModuleName, 2, "invalid ICS20 counterparty version")
	ErrInvalidChannelFlow           = sdkerrors.Register(ModuleName, 3, "invalid channel flow")
	ErrUnauthorizedIBCPacket        = sdkerrors.Register(ModuleName, 4, "unauthorized IBC packet")
	ErrFailedAcknowledgment         = sdkerrors.Register(ModuleName, 5, "failed acknowledgment")
	ErrNoActiveChannelPath          = sdkerrors.Register(ModuleName, 6, "no active channel path")
	ErrInvalidPacketSequence        = sdkerrors.Register(ModuleName, 7, "invalid packet sequence")
	ErrStablePriceNotFound          = sdkerrors.Register(ModuleName, 8, "stable price not found")
	ErrZeroStablePrice              = sdkerrors.Register(ModuleName, 9, "zero stable price")
	ErrDenomMappingExists           = sdkerrors.Register(ModuleName, 10, "denom mapping exists")
	ErrNegativeDenomPriceMultiplier = sdkerrors.Register(ModuleName, 11, "negative denom price multiplier")
	ErrUnauthorized                 = sdkerrors.Register(ModuleName, 12, "unauthorized")
	ErrFailedICQResponse            = sdkerrors.Register(ModuleName, 13, "failed ICQ response")
	ErrPendingRequest               = sdkerrors.Register(ModuleName, 14, "pending request")
)

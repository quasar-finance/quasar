package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qoracle module sentinel errors
var (
	ErrSample                     = sdkerrors.Register(ModuleName, 1100, "sample error")
	ErrInvalidStablePrice         = sdkerrors.Register(ModuleName, 200, "invalid stable price")
	ErrUnAuthorizedOracleClient   = sdkerrors.Register(ModuleName, 201, "unauthorized oracle client")
	ErrInvalidCounterpartyVersion = sdkerrors.Register(ModuleName, 2, "invalid ICS20 counterparty version")
	ErrInvalidChannelFlow         = sdkerrors.Register(ModuleName, 3, "invalid channel flow")
	ErrUnauthorizedIBCPacket      = sdkerrors.Register(ModuleName, 4, "unauthorized IBC packet")
	ErrFailedAcknowledgment       = sdkerrors.Register(ModuleName, 5, "failed acknowledgment")
	ErrNoActiveChannelPath        = sdkerrors.Register(ModuleName, 6, "no active channel path")
)

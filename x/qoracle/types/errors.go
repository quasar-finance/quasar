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
)

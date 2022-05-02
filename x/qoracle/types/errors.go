package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qoracle module sentinel errors
var (
	ErrSample             = sdkerrors.Register(ModuleName, 1100, "sample error")
	ErrInvalidStablePrice = sdkerrors.Register(ModuleName, 200, "invalid stable price")
)

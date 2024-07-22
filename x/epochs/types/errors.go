package types

import (
	errorsmod "cosmossdk.io/errors"
)

// ErrSample x/epochs module sentinel errors.
var (
	ErrSample = errorsmod.Register(ModuleName, 1100, "sample error")
)

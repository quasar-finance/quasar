package types

import (
	sdkerrors "cosmossdk.io/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled = sdkerrors.Register(SubModuleName, 2, "bandchain oracle module is disabled")
)

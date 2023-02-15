package types

import (
	"cosmossdk.io/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled = errors.Register(SubModuleName, 2, "bandchain oracle module is disabled")
)

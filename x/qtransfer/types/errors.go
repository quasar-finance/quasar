package types

import (
	errorsmod "cosmossdk.io/errors"
)

var (
	ErrInvalidMetadataFormat = errorsmod.New(ModuleName, 2, "invalid metadata format")
	ErrBadExecutionMsg       = "cannot execute contract: %v"
)

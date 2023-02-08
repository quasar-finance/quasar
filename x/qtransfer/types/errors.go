package types

import (
	sdkerrors "cosmossdk.io/errors"
)

var ErrInvalidMetadataFormat = sdkerrors.New(ModuleName, 2, "invalid metadata format")

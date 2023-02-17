package types

import (
	//	sdkerrors "cosmossdk.io/errors"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

var (
	ErrInvalidMetadataFormat = sdkerrors.New(ModuleName, 2, "invalid metadata format")
)

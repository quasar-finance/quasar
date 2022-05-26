package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/orion module sentinel errors
var (
	ErrSample                 = sdkerrors.Register(ModuleName, 1100, "sample error")
	ErrIcaMessageFailedInHost = sdkerrors.Register(ModuleName, 2000, "ica message failed in host")
)

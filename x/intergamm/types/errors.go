package types

import (
	"fmt"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// IBC channel sentinel errors
var (
	ErrAcknowledgementHookFailed = sdkerrors.Register(ModuleName, 2, "acknowledgement hook failed")
	ErrTimeoutHookFailed         = sdkerrors.Register(ModuleName, 3, "timeout hook failed")
	ErrInvalidZoneId             = sdkerrors.Register(ModuleName, 4, "invalid zone id")
	ErrInvalidDenom              = sdkerrors.Register(ModuleName, 5, "invalid zone id")
	ErrZoneInfoNotFound          = sdkerrors.Register(ModuleName, 6, "zone info not found")
	ErrICANotFound               = sdkerrors.Register(ModuleName, 7, "inter-chain account not found")
	ErrDenomNativeZoneIdNotFound = sdkerrors.Register(ModuleName, 8, "native zone id of the given denom not found")
)

func NewErrAcknowledgementHookFailed(msg string) error {
	return sdkerrors.Wrap(ErrAcknowledgementHookFailed, fmt.Sprintf("handling msg %s", msg))
}

func NewErrTimeoutHookFailed(msg string) error {
	return sdkerrors.Wrap(ErrTimeoutHookFailed, fmt.Sprintf("handling msg %s", msg))
}

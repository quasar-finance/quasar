package types

import (
	"fmt"

	"cosmossdk.io/errors"
)

// IBC channel sentinel errors
var (
	ErrAcknowledgementHookFailed = errors.Register(ModuleName, 2, "acknowledgement hook failed")
	ErrTimeoutHookFailed         = errors.Register(ModuleName, 3, "timeout hook failed")
	ErrInvalidZoneId             = errors.Register(ModuleName, 4, "invalid zone id")
	ErrInvalidDenom              = errors.Register(ModuleName, 5, "invalid zone id")
	ErrZoneInfoNotFound          = errors.Register(ModuleName, 6, "zone info not found")
	ErrICANotFound               = errors.Register(ModuleName, 7, "inter-chain account not found")
	ErrDenomNativeZoneIdNotFound = errors.Register(ModuleName, 8, "native zone id of the given denom not found")
)

func NewErrAcknowledgementHookFailed(msg string) error {
	return errors.Wrap(ErrAcknowledgementHookFailed, fmt.Sprintf("handling msg %s", msg))
}

func NewErrTimeoutHookFailed(msg string) error {
	return errors.Wrap(ErrTimeoutHookFailed, fmt.Sprintf("handling msg %s", msg))
}

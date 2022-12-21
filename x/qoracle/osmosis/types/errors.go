package types

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled            = sdkerrors.Register(SubModuleName, 2, "bandchain oracle module is disabled")
	ErrFailedICQResponse   = sdkerrors.Register(SubModuleName, 3, "failed ICQ response")
	ErrEpochNotFound       = sdkerrors.Register(SubModuleName, 4, "epoch not found")
	ErrGaugeWeightNotFound = sdkerrors.Register(SubModuleName, 5, "gauge weight not found")
)

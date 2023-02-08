package types

import (
	sdkerrors "cosmossdk.io/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled            = sdkerrors.Register(SubModuleName, 2, "osmosis oracle module is disabled")
	ErrInvalidChannelFlow  = sdkerrors.Register(SubModuleName, 3, "invalid message sent to channel end")
	ErrFailedICQResponse   = sdkerrors.Register(SubModuleName, 4, "failed ICQ response")
	ErrEpochNotFound       = sdkerrors.Register(SubModuleName, 5, "epoch not found")
	ErrGaugeWeightNotFound = sdkerrors.Register(SubModuleName, 6, "gauge weight not found")
)

package types

import (
	errorsmod "cosmossdk.io/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled            = errorsmod.Register(SubModuleName, 2, "osmosis oracle module is disabled")
	ErrInvalidChannelFlow  = errorsmod.Register(SubModuleName, 3, "invalid message sent to channel end")
	ErrFailedICQResponse   = errorsmod.Register(SubModuleName, 4, "failed ICQ response")
	ErrEpochNotFound       = errorsmod.Register(SubModuleName, 5, "epoch not found")
	ErrGaugeWeightNotFound = errorsmod.Register(SubModuleName, 6, "gauge weight not found")
	ErrOsmosisICQTimedOut  = errorsmod.Register(SubModuleName, 7, "osmosis icq request timeout")
)

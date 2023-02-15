package types

import (
	"cosmossdk.io/errors"
)

// IBC transfer sentinel errors
var (
	ErrDisabled            = errors.Register(SubModuleName, 2, "osmosis oracle module is disabled")
	ErrInvalidChannelFlow  = errors.Register(SubModuleName, 3, "invalid message sent to channel end")
	ErrFailedICQResponse   = errors.Register(SubModuleName, 4, "failed ICQ response")
	ErrEpochNotFound       = errors.Register(SubModuleName, 5, "epoch not found")
	ErrGaugeWeightNotFound = errors.Register(SubModuleName, 6, "gauge weight not found")
)

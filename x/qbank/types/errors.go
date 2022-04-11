package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qbank module sentinel errors
var (
	ErrDepositInvalidRiskProfile = sdkerrors.Register(ModuleName, 100, "invalid risk profile")
	ErrWithdrawInsufficientFunds = sdkerrors.Register(ModuleName, 200, "insufficient funds")
)

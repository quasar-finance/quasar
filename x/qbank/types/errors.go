package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qbank module sentinel errors
var (
	ErrInvalidVaultId            = sdkerrors.Register(ModuleName, 100, "invalid vault")
	ErrDepositInvalidRiskProfile = sdkerrors.Register(ModuleName, 200, "invalid risk profile")
	ErrWithdrawInsufficientFunds = sdkerrors.Register(ModuleName, 300, "insufficient funds")
)

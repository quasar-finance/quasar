package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qbank module sentinel errors
var (
	ErrInvalidVaultId            = sdkerrors.Register(ModuleName, 100, "invalid vault")
	ErrQbankNotEnabled           = sdkerrors.Register(ModuleName, 200, "qbank is not enabled")
	ErrDepositInvalidRiskProfile = sdkerrors.Register(ModuleName, 300, "invalid risk profile")
	ErrWithdrawInsufficientFunds = sdkerrors.Register(ModuleName, 400, "insufficient funds")
)

package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qbank module sentinel errors
var (
	ErrInvalidVaultId            = sdkerrors.Register(ModuleName, 100, "invalid vault")
	ErrInvalidLockupType         = sdkerrors.Register(ModuleName, 101, "invalid lockup type")
	ErrDepositInvalidRiskProfile = sdkerrors.Register(ModuleName, 102, "invalid risk profile")

	ErrQbankNotEnabled                = sdkerrors.Register(ModuleName, 200, "qbank is not enabled")
	ErrStablePriceNotAvailable        = sdkerrors.Register(ModuleName, 201, "stable price is not available")
	ErrInsufficientDollarDepositValue = sdkerrors.Register(ModuleName, 202, "insufficient dollar deposit value")
	ErrWithdrawInsufficientFunds      = sdkerrors.Register(ModuleName, 203, "insufficient funds")
	ErrReservedFieldLength            = sdkerrors.Register(ModuleName, 204, "invalid reserved field length")
)

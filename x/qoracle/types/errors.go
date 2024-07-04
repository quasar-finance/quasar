package types

import (
	errorsmod "cosmossdk.io/errors"
)

// DONTCOVER

// x/qoracle module sentinel errors
var (
	ErrNegativeDenomPriceMultiplier = errorsmod.Register(ModuleName, 2, "negative denom price multiplier")
	ErrPriceListOutdated            = errorsmod.Register(ModuleName, 3, "price list is outdated")
	ErrDenomPriceNotFound           = errorsmod.Register(ModuleName, 4, "symbol price not found")
	ErrRelativeDenomPriceNotFound   = errorsmod.Register(ModuleName, 5, "relative symbol price not found")
)

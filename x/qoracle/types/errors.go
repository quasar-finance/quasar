package types

// DONTCOVER

import (
	"cosmossdk.io/errors"
)

// x/qoracle module sentinel errors
var (
	ErrNegativeDenomPriceMultiplier = errors.Register(ModuleName, 2, "negative denom price multiplier")
	ErrPriceListOutdated            = errors.Register(ModuleName, 3, "price list is outdated")
	ErrDenomPriceNotFound           = errors.Register(ModuleName, 4, "symbol price not found")
)

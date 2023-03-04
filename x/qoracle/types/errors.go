package types

import "github.com/cosmos/cosmos-sdk/types/errors"

// DONTCOVER

// x/qoracle module sentinel errors
var (
	ErrNegativeDenomPriceMultiplier = errors.Register(ModuleName, 2, "negative denom price multiplier")
	ErrPriceListOutdated            = errors.Register(ModuleName, 3, "price list is outdated")
	ErrDenomPriceNotFound           = errors.Register(ModuleName, 4, "symbol price not found")
	ErrRelativeDenomPriceNotFound   = errors.Register(ModuleName, 5, "relative symbol price not found")
)

package types

// DONTCOVER

import (
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// x/qoracle module sentinel errors
var (
	ErrNegativeDenomPriceMultiplier = sdkerrors.Register(ModuleName, 2, "negative denom price multiplier")
	ErrPriceListOutdated            = sdkerrors.Register(ModuleName, 3, "price list is outdated")
	ErrDenomPriceNotFound           = sdkerrors.Register(ModuleName, 4, "symbol price not found")
)

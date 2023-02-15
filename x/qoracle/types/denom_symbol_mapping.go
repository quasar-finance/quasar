package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (mapping DenomSymbolMapping) Validate() error {
	if err := sdk.ValidateDenom(mapping.Denom); err != nil {
		return errors.Wrap(err, "mapping denom")
	}
	if err := sdk.ValidateDenom(mapping.OracleSymbol); err != nil {
		return errors.Wrap(err, "mapping oracle symbol")
	}
	if mapping.Multiplier.IsNegative() {
		return errors.Wrapf(ErrNegativeDenomPriceMultiplier, "multiplier of mapping can't be negative")
	}
	return nil
}

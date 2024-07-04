package types

import (
	errorsmod "cosmossdk.io/errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (mapping DenomSymbolMapping) Validate() error {
	if err := sdk.ValidateDenom(mapping.Denom); err != nil {
		return errorsmod.Wrap(err, "mapping denom")
	}
	if err := sdk.ValidateDenom(mapping.OracleSymbol); err != nil {
		return errorsmod.Wrap(err, "mapping oracle symbol")
	}
	if mapping.Multiplier.IsNegative() {
		return errorsmod.Wrapf(ErrNegativeDenomPriceMultiplier, "multiplier of mapping can't be negative")
	}
	return nil
}

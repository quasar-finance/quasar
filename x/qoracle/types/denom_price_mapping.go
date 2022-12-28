package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (mapping DenomPriceMapping) Validate() error {
	if err := sdk.ValidateDenom(mapping.Denom); err != nil {
		return sdkerrors.Wrap(err, "mapping denom")
	}
	if err := sdk.ValidateDenom(mapping.OracleSymbol); err != nil {
		return sdkerrors.Wrap(err, "mapping oracle symbol")
	}
	if mapping.Multiplier.IsNegative() {
		return sdkerrors.Wrapf(ErrNegativeDenomPriceMultiplier, "multiplier of mapping can't be negative")
	}
	return nil
}

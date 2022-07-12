package types

import sdk "github.com/cosmos/cosmos-sdk/types"

func (mapping DenomPriceMapping) ValidateBasic() error {
	if err := sdk.ValidateDenom(mapping.Denom); err != nil {
		return err
	}
	if mapping.Multiplier.IsNegative() {
		return ErrNegativeDenomPriceMultiplier
	}
	return nil
}

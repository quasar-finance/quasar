package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (m PoolMetrics) Validate() error {
	if apy, err := sdk.NewDecFromStr(m.APY); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid APY '%s': %s", m.APY, err.Error())
	} else if apy.IsNegative() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "negative APY '%s'", m.APY)
	}
	if _, err := sdk.ParseDecCoin(m.TVL); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid TVL '%s': %s", m.TVL, err.Error())
	}
	return nil
}

func (m PoolMetrics) IsValid() bool {
	return m.Validate() == nil
}

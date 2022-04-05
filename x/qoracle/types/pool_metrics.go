package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (m PoolMetrics) Validate() error {
	if apy, err := sdk.NewDecFromStr(m.HighestAPY); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid HighestAPY '%s': %s", m.HighestAPY, err.Error())
	} else if apy.IsNegative() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "negative HighestAPY '%s'", m.HighestAPY)
	}
	if _, err := sdk.ParseDecCoin(m.TVL); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid TVL '%s': %s", m.TVL, err.Error())
	}
	if m.GaugeAPYs != nil {
		for i, ga := range m.GaugeAPYs {
			if m.GaugeAPYs[i] == nil {
				return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "nil GaugeAPY at index %d", i)
			}
			if err := ga.Validate(); err != nil {
				return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid GaugeAPY at index %d: %s", i, err.Error())
			}
		}
	}
	return nil
}

func (m PoolMetrics) IsValid() bool {
	return m.Validate() == nil
}

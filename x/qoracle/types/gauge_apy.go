package types

import (
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

func (ga GaugeAPY) Validate() error {
	if duration, err := time.ParseDuration(ga.Duration); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid Duration '%s': %s", ga.Duration, err.Error())
	} else if duration <= 0 {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "Duration '%s' must be positive", ga.Duration)
	}
	if apy, err := sdk.NewDecFromStr(ga.APY); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "invalid APY '%s': %s", ga.APY, err.Error())
	} else if apy.IsNegative() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidType, "negative APY '%s'", ga.APY)
	}
	return nil
}

func (ga GaugeAPY) IsValid() bool {
	return ga.Validate() == nil
}

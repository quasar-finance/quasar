package balancer

import (
	"errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	gammtypes "github.com/abag/quasarnode/x/intergamm/types/osmosis/v9/gamm"
)

var (
	_ gammtypes.PoolI                  = &Pool{}
	_ gammtypes.PoolAmountOutExtension = &Pool{}
)

// ValidateUserSpecifiedWeight ensures that a weight that is provided from user-input anywhere
// for creating a pool obeys the expected guarantees.
// Namely, that the weight is in the range [1, MaxUserSpecifiedWeight)
func ValidateUserSpecifiedWeight(weight sdk.Int) error {
	if !weight.IsPositive() {
		return sdkerrors.Wrap(gammtypes.ErrNotPositiveWeight, weight.String())
	}

	if weight.GTE(MaxUserSpecifiedWeight) {
		return sdkerrors.Wrap(gammtypes.ErrWeightTooLarge, weight.String())
	}
	return nil
}

func (params PoolParams) Validate(poolWeights []PoolAsset) error {
	if params.ExitFee.IsNegative() {
		return gammtypes.ErrNegativeExitFee
	}

	if params.ExitFee.GTE(sdk.OneDec()) {
		return gammtypes.ErrTooMuchExitFee
	}

	if params.SwapFee.IsNegative() {
		return gammtypes.ErrNegativeSwapFee
	}

	if params.SwapFee.GTE(sdk.OneDec()) {
		return gammtypes.ErrTooMuchSwapFee
	}

	if params.SmoothWeightChangeParams != nil {
		targetWeights := params.SmoothWeightChangeParams.TargetPoolWeights
		// Ensure it has the right number of weights
		if len(targetWeights) != len(poolWeights) {
			return gammtypes.ErrPoolParamsInvalidNumDenoms
		}
		// Validate all user specified weights
		for _, v := range targetWeights {
			err := ValidateUserSpecifiedWeight(v.Weight)
			if err != nil {
				return err
			}
		}
		// Ensure that all the target weight denoms are same as pool asset weights
		sortedTargetPoolWeights := SortPoolAssetsOutOfPlaceByDenom(targetWeights)
		sortedPoolWeights := SortPoolAssetsOutOfPlaceByDenom(poolWeights)
		for i, v := range sortedPoolWeights {
			if sortedTargetPoolWeights[i].Token.Denom != v.Token.Denom {
				return gammtypes.ErrPoolParamsInvalidDenom
			}
		}

		// No start time validation needed

		// We do not need to validate InitialPoolWeights, as we set that ourselves
		// in setInitialPoolParams

		// TODO: Is there anything else we can validate for duration?
		if params.SmoothWeightChangeParams.Duration <= 0 {
			return errors.New("params.SmoothWeightChangeParams must have a positive duration")
		}
	}

	return nil
}

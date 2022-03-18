package balancer

import (
	"errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	// 	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	"github.com/abag/quasarnode/x/gamm/types"
	osmosis_balancer_pool "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	osmosis_gamm_types "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
)

func (pool BalancerPool) Validate() error {
	err := types.ValidatePoolAssets(pool.PoolAssets)
	if err != nil {
		return err
	}

	err = pool.PoolParams.Validate(pool.PoolAssets)
	if err != nil {
		return err
	}

	// validation for future owner
	if err = osmosis_balancer_pool.ValidateFutureGovernor(pool.FuturePoolGovernor); err != nil {
		return err
	}

	return nil
}

func (params BalancerPoolParams) Validate(poolWeights []types.PoolAsset) error {
	if params.ExitFee.IsNegative() {
		return osmosis_gamm_types.ErrNegativeExitFee
	}

	if params.ExitFee.GTE(sdk.OneDec()) {
		return osmosis_gamm_types.ErrTooMuchExitFee
	}

	if params.SwapFee.IsNegative() {
		return osmosis_gamm_types.ErrNegativeSwapFee
	}

	if params.SwapFee.GTE(sdk.OneDec()) {
		return osmosis_gamm_types.ErrTooMuchSwapFee
	}

	if params.SmoothWeightChangeParams != nil {
		targetWeights := params.SmoothWeightChangeParams.TargetPoolWeights
		// Ensure it has the right number of weights
		if len(targetWeights) != len(poolWeights) {
			return osmosis_gamm_types.ErrPoolParamsInvalidNumDenoms
		}
		// Validate all user specified weights
		for _, v := range targetWeights {
			err := osmosis_gamm_types.ValidateUserSpecifiedWeight(v.Weight)
			if err != nil {
				return err
			}
		}
		// Ensure that all the target weight denoms are same as pool asset weights
		sortedTargetPoolWeights := types.SortPoolAssetsOutOfPlaceByDenom(targetWeights)
		sortedPoolWeights := types.SortPoolAssetsOutOfPlaceByDenom(poolWeights)
		for i, v := range sortedPoolWeights {
			if sortedTargetPoolWeights[i].Token.Denom != v.Token.Denom {
				return osmosis_gamm_types.ErrPoolParamsInvalidDenom
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

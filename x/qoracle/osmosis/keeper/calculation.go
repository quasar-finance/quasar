package keeper

import (
	"fmt"
	"time"

	//	sdkerrors "cosmossdk.io/errors"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	epochtypes "github.com/quasarlabs/quasarnode/osmosis/epochs/types"
	balancerpool "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	poolincentivestypes "github.com/quasarlabs/quasarnode/osmosis/pool-incentives/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

const Year = 365 * 24 * time.Hour

// apyCalculator caches all the pre calculations needed for calculating pool APYs in batch.
type apyCalculator struct {
	distrInfo            poolincentivestypes.DistrInfo
	incentivizedPools    []poolincentivestypes.IncentivizedPool
	mintTokenPrice       sdk.Dec
	annualPoolIncentives sdk.Dec
}

func (k Keeper) newAPYCalculator(ctx sdk.Context) (apyCalculator, error) {
	distrInfo := k.GetDistrInfo(ctx)
	epochProvisions := k.GetMintEpochProvisions(ctx)

	mintParams := k.GetMintParams(ctx)
	poolIncentivesProportion := mintParams.DistributionProportions.PoolIncentives
	mintTokenPrice, err := k.qoracleKeeper.GetDenomPrice(ctx, mintParams.MintDenom)
	if err != nil {
		return apyCalculator{}, sdkerrors.Wrap(err, "could not fetch the price of mint denom")
	}
	mintEpoch, found := k.findOsmosisEpochByIdentifier(ctx, mintParams.EpochIdentifier)
	if !found {
		return apyCalculator{}, sdkerrors.Wrap(types.ErrEpochNotFound, fmt.Sprintf("could not find osmosis mint, epoch identifier: %s", mintParams.EpochIdentifier))
	}

	// Number of mint epochs occurrence in a year
	annualMintEpochs := Year.Nanoseconds() / mintEpoch.Duration.Nanoseconds()
	annualProvisions := epochProvisions.MulInt64(annualMintEpochs)
	// Annual provisions share to incentivize pools is equal to "annualProvisions * poolIncentivesProportion"
	annualPoolIncentives := annualProvisions.Mul(poolIncentivesProportion)

	return apyCalculator{
		distrInfo:            distrInfo,
		incentivizedPools:    k.GetIncentivizedPools(ctx),
		mintTokenPrice:       mintTokenPrice,
		annualPoolIncentives: annualPoolIncentives,
	}, nil
}

// Calculate the pool APY given the pool itself and it's TVL.
func (apyc apyCalculator) Calculate(ctx sdk.Context, pool balancerpool.Pool, poolTVL sdk.Dec) (sdk.Dec, error) {
	// Calculate the pool total weight from it's incentivized gauges
	poolTotalWeight := sdk.ZeroInt()
	for _, incentive := range apyc.incentivizedPools {
		if incentive.PoolId == pool.Id {
			gaugeWeight, found := findGaugeWeight(ctx, incentive.GaugeId, apyc.distrInfo)
			if !found {
				return sdk.ZeroDec(), sdkerrors.Wrap(types.ErrGaugeWeightNotFound, fmt.Sprintf("gauge id: %d", incentive.GaugeId))
			}
			poolTotalWeight = poolTotalWeight.Add(gaugeWeight)
		}
	}

	// Total annual provision share (including all gauges) of the requested pool in $
	// is equal to "annualPoolIncentives * poolTotalWeight / distrInfo.TotalWeight * mintTokenPrice"
	poolAnnualProvisions := apyc.annualPoolIncentives.MulInt(poolTotalWeight).QuoInt(apyc.distrInfo.TotalWeight).Mul(apyc.mintTokenPrice)
	// APY of the requested pool is equal to "(poolAnnualProvisions / poolTVL) * 100"
	poolAPY := poolAnnualProvisions.Quo(poolTVL).Mul(sdk.NewDec(100))
	return poolAPY, nil
}

func (k Keeper) CalculatePoolTVL(ctx sdk.Context, pool balancerpool.Pool) (sdk.Dec, error) {
	tvl := sdk.ZeroDec()

	for _, asset := range pool.PoolAssets {
		price, err := k.qoracleKeeper.GetDenomPrice(ctx, asset.Token.Denom)
		if err != nil {
			return sdk.ZeroDec(), err
		}

		tvl = tvl.Add(sdk.NewDecFromInt(asset.Token.Amount).Mul(price))
	}
	return tvl, nil
}

// findOsmosisEpochByIdentifier iterates over all osmosis epochs and returns the epoch with given identifier if exists.
func (k Keeper) findOsmosisEpochByIdentifier(ctx sdk.Context, identifier string) (epochtypes.EpochInfo, bool) {
	for _, epoch := range k.GetEpochsInfo(ctx) {
		if epoch.Identifier == identifier {
			return epoch, true
		}
	}
	return epochtypes.EpochInfo{}, false
}

// findGaugeWeight iterates over distrInfo.Records and returns the weight of record is it finds and record with given gaugeId.
func findGaugeWeight(ctx sdk.Context, gaugeId uint64, distrInfo poolincentivestypes.DistrInfo) (sdk.Int, bool) {
	for _, record := range distrInfo.Records {
		if record.GaugeId == gaugeId {
			return record.Weight, true
		}
	}
	return sdk.ZeroInt(), false
}

func extractPoolAssets(pool balancerpool.Pool) sdk.Coins {
	coins := make([]sdk.Coin, len(pool.PoolAssets))
	for i, asset := range pool.PoolAssets {
		coins[i] = sdk.NewCoin(asset.Token.Denom, asset.Token.Amount)
	}
	return sdk.NewCoins(coins...)
}

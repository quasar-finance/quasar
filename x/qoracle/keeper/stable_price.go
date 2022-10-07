package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// SetStablePrice set the stable price for the symbol
func (k Keeper) SetStablePrice(ctx sdk.Context, symbol string, price sdk.Dec) {
	denomMapping := k.GetDenomPriceMappings(ctx)
	denomMapping = append(denomMapping, types.DenomPriceMapping{
		Denom:       symbol,
		OracleDenom: symbol,
		Multiplier:  sdk.NewDec(1),
	})
	k.SetDenomPriceMappings(ctx, denomMapping)

	op := k.GetOraclePrices(ctx)
	op.Prices = op.Prices.Add(sdk.NewDecCoinFromDec(symbol, price))
	k.SetOraclePrices(ctx, op)
}

// GetStablePrice get the stable denom for the symbol
func (k Keeper) GetStablePrice(ctx sdk.Context, symbol string) (price sdk.Dec, found bool) {
	denomMapping := k.GetDenomPriceMappings(ctx)
	for _, d := range denomMapping {
		if d.Denom == symbol {
			op := k.GetOraclePrices(ctx)
			amount := op.Prices.AmountOf(d.OracleDenom)
			if amount.IsZero() {
				return price, false
			}
			return amount.Mul(d.Multiplier), true
		}
	}
	return price, false
}

// GetRelativeStablePrice calculates how many denomOut is equivalent to one denomIn.
func (k Keeper) GetRelativeStablePrice(ctx sdk.Context, denomIn, denomOut string) (sdk.Dec, error) {
	priceIn, found := k.GetStablePrice(ctx, denomIn)
	if !found {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrStablePriceNotFound, "stable price not found for denom %s", denomIn)
	}
	priceOut, found := k.GetStablePrice(ctx, denomOut)
	if !found {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrStablePriceNotFound, "stable price not found for denom %s", denomOut)
	}
	if priceOut.IsZero() {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrZeroStablePrice, "zero stable price for denom %s", denomOut)
	}
	return priceIn.Quo(priceOut), nil
}

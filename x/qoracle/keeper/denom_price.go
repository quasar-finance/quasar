package keeper

import (
	"time"

	//sdkerrors "cosmossdk.io/errors"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetDenomPrice get the stable denom for the denom
func (k Keeper) GetDenomPrice(ctx sdk.Context, denom string) (sdk.Dec, error) {
	updatedAt, err := k.GetDenomPricesUpdatedAt(ctx)
	if err != nil {
		return sdk.ZeroDec(), err
	}
	// Check whether denom prices are outdated
	if ctx.BlockTime().Before(updatedAt.Add(time.Duration(k.GetPriceListExpDuration(ctx)))) {
		return sdk.ZeroDec(), types.ErrPriceListOutdated
	}

	memStore := ctx.KVStore(k.memKey)
	priceBz := memStore.Get(types.GetDenomPriceKey(denom))
	if priceBz == nil {
		return sdk.Dec{}, sdkerrors.Wrapf(types.ErrDenomPriceNotFound, "denom: %s", denom)
	}

	var price sdk.Dec
	if err := price.Unmarshal(priceBz); err != nil {
		return sdk.Dec{}, err
	}
	return price, nil
}

// GetDenomPricesUpdatedAt get the last time denom prices were updated.
func (k Keeper) GetDenomPricesUpdatedAt(ctx sdk.Context) (time.Time, error) {
	memStore := ctx.KVStore(k.memKey)
	if !memStore.Has(types.KeyMemDenomPricesUpdatedAt) {
		return time.Time{}, nil
	}

	updatedAt, err := sdk.ParseTimeBytes(memStore.Get(types.KeyMemDenomPricesUpdatedAt))
	if err != nil {
		return time.Time{}, sdkerrors.Wrap(err, "failed to parse denom prices updated at")
	}
	return updatedAt, nil
}

// GetRelativeDenomPrice get the relative price of token with denomIn in denomOut.
func (k Keeper) GetRelativeDenomPrice(ctx sdk.Context, denomIn, denomOut string) (sdk.Dec, error) {
	denomInPrice, err := k.GetDenomPrice(ctx, denomIn)
	if err != nil {
		return sdk.ZeroDec(), err
	}
	denomOutPrice, err := k.GetDenomPrice(ctx, denomOut)
	if err != nil {
		return sdk.ZeroDec(), err
	}

	if denomOutPrice.IsZero() || denomOutPrice.IsNil() || denomOutPrice.IsNegative() {
		// In this case, division by denomOutPrice is risky
		return sdk.ZeroDec(), sdkerrors.Wrapf(types.ErrRelativeDenomPriceNotFound,
			"denomInPrice: %s", denomInPrice.String(),
			"denomOutPrice: %s", denomOutPrice.String())
	}

	// Can this cause - non determinism
	return denomInPrice.Quo(denomOutPrice), nil
}

package keeper

import (
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// NOTE -
// Denom price will be updated in the future in their respective memory store key.
// using either bandchain, chain link or USDC pool from osmosis. Code is kept for future use.

// GetDenomPrice get the stable price for the denom
func (k Keeper) GetDenomPrice(ctx sdk.Context, denom string) (sdk.Dec, error) {
	_, err := k.GetDenomPricesUpdatedAt(ctx)
	if err != nil {
		// Last update time not found.
		return sdk.ZeroDec(), err
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
			"denomInPrice: %s, denomOutPrice : %s", denomInPrice.String(), denomOutPrice.String())
	}

	return denomInPrice.Quo(denomOutPrice), nil
}

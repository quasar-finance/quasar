package keeper

import (
	"github.com/abag/quasarnode/x/qoracle/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// AddDenomMapping adds a new denom mapping to the store, returns an error if the mapping already exists.
func (k Keeper) AddDenomMapping(ctx sdk.Context, mapping types.DenomPriceMapping) error {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyDenomPriceMappingPrefix)
	if store.Has([]byte(mapping.Denom)) {
		return types.ErrDenomMappingExists
	}

	store.Set([]byte(mapping.Denom), k.cdc.MustMarshal(&mapping))
	return nil
}

// GetDenomMapping returns the denom mapping for the given denom.
func (k Keeper) GetDenomMapping(ctx sdk.Context, denom string) (types.DenomPriceMapping, bool) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.KeyDenomPriceMappingPrefix)
	bz := store.Get([]byte(denom))
	if bz == nil {
		return types.DenomPriceMapping{}, false
	}
	var mapping types.DenomPriceMapping
	k.cdc.MustUnmarshal(bz, &mapping)
	return mapping, true
}

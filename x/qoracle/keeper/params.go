package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// GetParams get all parameters as types.Params
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.GetPriceListExpDuration(ctx),
		k.GetSymbolMappings(ctx),
	)
}

// SetParams set the params
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	// TODO - Sort the params.Mappings before storing or atleast validate.
	k.paramSpace.SetParamSet(ctx, &params)
}

// GetPacketTimeoutTimestamp retrieves the price list expiration duration from the param space
func (k Keeper) GetPriceListExpDuration(ctx sdk.Context) (res uint64) {
	k.paramSpace.Get(ctx, types.KeyDenomPricesExpDuration, &res)
	return
}

// GetPacketTimeoutTimestamp retrieves the price list expiration duration from the param space
func (k Keeper) GetSymbolMappings(ctx sdk.Context) (res []types.DenomSymbolMapping) {
	k.paramSpace.Get(ctx, types.KeyDenomSymbolMapping, &res)
	return
}

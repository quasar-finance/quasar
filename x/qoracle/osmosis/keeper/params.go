package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v6/modules/core/02-client/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
)

// GetParams returns the total set of module parameters.
func (k Keeper) GetParams(ctx sdk.Context) types.Params {
	return types.NewParams(
		k.IsEnabled(ctx),
		k.GetEpochIdentifier(ctx),
		k.GetAuthorizedChannel(ctx),
		k.GetPacketTimeoutHeight(ctx),
		k.GetPacketTimeoutTimestamp(ctx),
	)
}

// SetParams sets the total set of module parameters.
func (k Keeper) SetParams(ctx sdk.Context, params types.Params) {
	k.paramSpace.SetParamSet(ctx, &params)
}

// IsEnabled retrieves the enabled boolean from the paramstore
func (k Keeper) IsEnabled(ctx sdk.Context) (res bool) {
	k.paramSpace.Get(ctx, types.KeyEnabled, &res)
	return
}

// GetEpochIdentifier retrieves the epoch identifier from the paramstore
func (k Keeper) GetEpochIdentifier(ctx sdk.Context) (res string) {
	k.paramSpace.Get(ctx, types.KeyEpochIdentifier, &res)
	return
}

// GetAuthorizedChannel retrieves the authorized channel from the paramstore
func (k Keeper) GetAuthorizedChannel(ctx sdk.Context) (res string) {
	k.paramSpace.Get(ctx, types.KeyAuthorizedChannel, &res)
	return
}

// GetPacketTimeoutHeight retrieves the timeout height from the paramstore
func (k Keeper) GetPacketTimeoutHeight(ctx sdk.Context) (res clienttypes.Height) {
	k.paramSpace.Get(ctx, types.KeyPacketTimeoutHeight, &res)
	return
}

// GetPacketTimeoutTimestamp retrieves the timeout timestamp from the paramstore
func (k Keeper) GetPacketTimeoutTimestamp(ctx sdk.Context) (res uint64) {
	k.paramSpace.Get(ctx, types.KeyPacketTimeoutTimestamp, &res)
	return
}

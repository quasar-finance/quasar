package types

import sdk "github.com/cosmos/cosmos-sdk/types"

// QOracle is an interface of qoracle keeper.
type QOracle interface {
	NotifyPoolsUpdate(ctx sdk.Context)
	GetDenomPrice(ctx sdk.Context, denom string) (sdk.Dec, error)
}

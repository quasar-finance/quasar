package types

import sdk "github.com/cosmos/cosmos-sdk/types"

// QOracle is an interface of qoracle keeper.
type QOracle interface {
	NotifySymbolPricesUpdate(ctx sdk.Context)
}

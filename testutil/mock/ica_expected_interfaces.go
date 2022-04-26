package mock

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

type ICAControllerKeeper interface {
	GetInterchainAccountAddress(ctx sdk.Context, connectionID, portID string) (string, bool)
}

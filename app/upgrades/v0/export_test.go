package v0

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	qvestingkeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
)

func SetQVestingParams(ctx sdk.Context, icqKeeper *qvestingkeeper.Keeper) {
	setQVestingParams(ctx, icqKeeper)
}

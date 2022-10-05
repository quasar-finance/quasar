package keeper_test

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qbank/keeper"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
)

func setupMsgServer(ctx sdk.Context, k keeper.Keeper) (types.MsgServer, context.Context) {
	return keeper.NewMsgServerImpl(k), sdk.WrapSDKContext(ctx)
}

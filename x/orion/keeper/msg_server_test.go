package keeper_test

import (
	"context"
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func setupMsgServer(t *testing.T) (types.MsgServer, context.Context) {
	ctx, k := keepertest.NewTestSetup(t).GetOrionKeeper()
	return keeper.NewMsgServerImpl(k), sdk.WrapSDKContext(ctx)
}

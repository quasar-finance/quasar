package keeper_test

import (
	"context"
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/orion/keeper"
	"github.com/abag/quasarnode/x/orion/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func setupMsgServer(t *testing.T) (types.MsgServer, context.Context) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.OrionKeeper
	return keeper.NewMsgServerImpl(k), sdk.WrapSDKContext(ctx)
}

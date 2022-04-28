package keeper_test

import (
	"context"
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func setupMsgServer(t testing.TB) (types.MsgServer, context.Context) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.QoracleKeeper
	return keeper.NewMsgServerImpl(k), sdk.WrapSDKContext(ctx)
}

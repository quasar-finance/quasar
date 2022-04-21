package keeper_test

import (
	"context"
	"testing"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qoracle/keeper"
	"github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func setupMsgServer(t testing.TB) (types.MsgServer, context.Context) {
	ctx, k := keepertest.NewTestSetup(t).GetQoracleKeeper()
	return keeper.NewMsgServerImpl(k), sdk.WrapSDKContext(ctx)
}

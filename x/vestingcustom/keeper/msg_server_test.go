package keeper_test

import (
	"context"
	"testing"

	sdk "github.com/cosmos/cosmos-sdk/types"
	keepertest "quasar/testutil/keeper"
	"github.com/quasarlabs/quasarnode/x/vestingcustom/keeper"
	"github.com/quasarlabs/quasarnode/x/vestingcustom/types"
)

func setupMsgServer(t testing.TB) (types.MsgServer, context.Context) {
	k, ctx := keepertest.VestingcustomKeeper(t)
	return keeper.NewMsgServerImpl(*k), sdk.WrapSDKContext(ctx)
}

package keeper_test

import (
	"context"

	keepertest "github.com/abag/quasarnode/testutil/keeper"
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func setupMsgServer(tks *keepertest.TestKeeperState) (types.MsgServer, context.Context) {
	k, ctx := keepertest.QbankKeeperExistingState(tks)
	return keeper.NewMsgServerImpl(*k), sdk.WrapSDKContext(ctx)
}

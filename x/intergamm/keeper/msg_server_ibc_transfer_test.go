package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	"github.com/golang/mock/gomock"
)

func TestIbcTransfer(t *testing.T) {
	ctl := gomock.NewController(t)
	defer ctl.Finish()
	// mocks := mocktest.NewTestMocks(t, ctl)
	// keepers := keepertest.NewTestSetup(t)
	// ctx, keeper := keepers.GetInterGammKeeper()
	// srv, srvCtx := setupMsgServer(ctx, keeper)

	sender := sample.AccAddress()
	receiver := sample.AccAddress()

	msg := types.NewMsgIbcTransfer(
		sender.String(),
		"con",
		uint64(10),
		"transfer",
		"channel-0",
		sdk.NewCoin("qsr", sdk.NewInt(42)),
		receiver.String(),
		clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0},
		uint64(10),
	)

	println(msg)
	// TODO
	// _, err := srv.IbcTransfer(srvCtx, msg)
	// require.NoError(t, err)
}

package keeper_test

import (
	"fmt"
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/require"
)

func TestIbcTransfer(t *testing.T) {
	var err error
	ctl := gomock.NewController(t)
	defer ctl.Finish()
	setup := testutil.NewTestSetup(t, ctl)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	srv, srvCtx := setupMsgServer(ctx, k)

	sender := sample.AccAddress()
	receiver := sample.AccAddress()

	msg := types.NewMsgIbcTransfer(
		sender.String(),
		"testConnectionId",
		uint64(10),
		"transfer",
		"channel-0",
		sdk.NewCoin("qsr", sdk.NewInt(42)),
		receiver.String(),
		clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0},
		uint64(10),
	)

	portId := fmt.Sprintf("icacontroller-%s", sender.String())

	capPath := host.ChannelCapabilityPath(portId, "channel-0")
	cap, err := setup.Keepers.InterGammKeeper.ScopedKeeper.NewCapability(ctx, capPath)
	require.NoError(t, err)
	setup.Keepers.InterGammKeeper.ClaimCapability(ctx, capabilitytypes.NewCapability(cap.GetIndex()), capPath)

	// Expected mocks
	gomock.InOrder(
		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetInterchainAccountAddress(ctx, "testConnectionId", portId).
			Return("bla1", true),

		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetActiveChannelID(ctx, "testConnectionId", portId).
			Return("channel-0", true),

		// TODO expect a specific packet
		setup.Mocks.ICAControllerKeeperMock.EXPECT().SendTx(ctx, gomock.Any(), "testConnectionId", portId, gomock.Any(), uint64(10)).
			Return(uint64(42), nil),
	)

	_, err = srv.IbcTransfer(srvCtx, msg)
	require.NoError(t, err)
}

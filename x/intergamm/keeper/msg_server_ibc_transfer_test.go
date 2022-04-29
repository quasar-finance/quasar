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
	connectionId := "testConnectionId"
	connectionTimeout := uint64(10)
	portId := fmt.Sprintf("icacontroller-%s", sender.String())
	channelId := "channel-0"

	msg := types.NewMsgIbcTransfer(
		sender.String(),
		connectionId,
		connectionTimeout,
		portId,
		channelId,
		sdk.NewCoin("qsr", sdk.NewInt(42)),
		receiver.String(),
		clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0},
		uint64(10),
	)

	capPath := host.ChannelCapabilityPath(portId, channelId)
	cap, err := setup.Keepers.InterGammKeeper.ScopedKeeper.NewCapability(ctx, capPath)
	require.NoError(t, err)
	setup.Keepers.InterGammKeeper.ClaimCapability(ctx, capabilitytypes.NewCapability(cap.GetIndex()), capPath)

	// Expected mocks
	gomock.InOrder(
		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetInterchainAccountAddress(ctx, connectionId, portId).
			Return("", true),

		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetActiveChannelID(ctx, connectionId, portId).
			Return(channelId, true),

		// TODO expect a specific packet
		setup.Mocks.ICAControllerKeeperMock.EXPECT().SendTx(ctx, gomock.Any(), connectionId, portId, gomock.Any(), connectionTimeout).
			Return(uint64(42), nil),
	)

	resp, err := srv.IbcTransfer(srvCtx, msg)
	require.NoError(t, err)
	require.Equal(t, types.MsgIbcTransferResponse{}, *resp)
}

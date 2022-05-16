package keeper_test

import (
	"fmt"
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/testutil/sample"
	"github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/require"
)

func icaPacket(t *testing.T, setup *testutil.TestSetup, msgs []sdk.Msg) icatypes.InterchainAccountPacketData {
	data, err := icatypes.SerializeCosmosTx(setup.Cdc, msgs)
	require.NoError(t, err)

	return icatypes.InterchainAccountPacketData{
		Type: icatypes.EXECUTE_TX,
		Data: data,
	}
}

func TestIbcTransfer(t *testing.T) {
	var err error
	ctl := gomock.NewController(t)
	defer ctl.Finish()
	setup := testutil.NewTestSetup(t, ctl)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	srv, srvCtx := setupMsgServer(ctx, k)

	// Test data
	sender := sample.AccAddress()
	receiver := sample.AccAddress()
	icaTestAddress := "icaTestAddress"
	connectionId := "testConnectionId"
	connectionTimeout := uint64(ctx.BlockTime().UnixNano()) + keeper.DefaultSendTxRelativeTimeoutTimestamp
	portId := fmt.Sprintf("icacontroller-%s", sender.String())
	channelId := "channel-0"
	coin := sdk.NewCoin("qsr", sdk.NewInt(42))
	transferTimeoutHeight := clienttypes.Height{RevisionNumber: 0, RevisionHeight: 0}
	transferTimeout := connectionTimeout
	msg := types.NewMsgIbcTransfer(
		sender.String(),
		connectionId,
		connectionTimeout,
		portId,
		channelId,
		coin,
		receiver.String(),
		transferTimeoutHeight,
		transferTimeout,
	)

	// Setup capability
	capPath := host.ChannelCapabilityPath(portId, channelId)
	cap, err := setup.Keepers.InterGammKeeper.NewCapability(ctx, capPath)
	require.NoError(t, err)

	// Expected mocks
	gomock.InOrder(
		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetInterchainAccountAddress(ctx, connectionId, portId).
			Return(icaTestAddress, true),

		setup.Mocks.ICAControllerKeeperMock.EXPECT().GetActiveChannelID(ctx, connectionId, portId).
			Return(channelId, true),

		setup.Mocks.ICAControllerKeeperMock.EXPECT().SendTx(
			ctx,
			cap,
			connectionId,
			portId,
			icaPacket(t, setup, []sdk.Msg{
				&ibctransfertypes.MsgTransfer{
					SourcePort:       portId,
					SourceChannel:    channelId,
					Token:            coin,
					Sender:           icaTestAddress,
					Receiver:         receiver.String(),
					TimeoutHeight:    transferTimeoutHeight,
					TimeoutTimestamp: transferTimeout,
				},
			}),
			connectionTimeout,
		).Return(uint64(0), nil),
	)

	resp, err := srv.IbcTransfer(srvCtx, msg)
	require.NoError(t, err)
	require.Equal(t, types.MsgIbcTransferResponse{}, *resp)
}

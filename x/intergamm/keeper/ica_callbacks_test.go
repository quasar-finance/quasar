package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/intergamm/types"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	"github.com/stretchr/testify/require"
)

// func mustB64DecodeString(t *testing.T, str string) []byte {
// 	b, err := base64.StdEncoding.DecodeString("Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w=")
// 	require.NoError(t, err)

// 	return b
// }

func _makeIcaPacket(t *testing.T, cdc codec.Codec) func(msg sdk.Msg) icatypes.InterchainAccountPacketData {
	return func(msg sdk.Msg) icatypes.InterchainAccountPacketData {
		data, err := icatypes.SerializeCosmosTx(cdc, []sdk.Msg{msg})
		require.NoError(t, err)

		return icatypes.InterchainAccountPacketData{
			Type: icatypes.EXECUTE_TX,
			Data: data,
		}
	}
}

func makeAck(t *testing.T, req sdk.Msg, res proto.Message) channeltypes.Acknowledgement {
	resData, err := proto.Marshal(res)
	require.NoError(t, err)

	txMsgData := &sdk.TxMsgData{
		Data: []*sdk.MsgData{
			{
				MsgType: sdk.MsgTypeURL(req),
				Data:    resData,
			},
		},
	}
	ackData, err := proto.Marshal(txMsgData)
	require.NoError(t, err)

	return channeltypes.NewResultAcknowledgement(ackData)
}

// func makeErrorAck(t *testing.T, errorStr string) channeltypes.Acknowledgement {
// 	return channeltypes.NewErrorAcknowledgement(errorStr)
// }

func TestParseAck(t *testing.T) {
	testCases := []struct {
		name     string
		ack      channeltypes.Acknowledgement
		req      *gammbalancer.MsgCreateBalancerPool
		resp     *gammbalancer.MsgCreateBalancerPoolResponse
		errorStr string
	}{
		{
			name: "valid",
			ack:  makeAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
			// ack: mustB64DecodeString(t, "Ci0KKy9vc21vc2lzLmdhbW0udjFiZXRhMS5Nc2dDcmVhdGVCYWxhbmNlclBvb2w="),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "",
		},
		{
			name:     "invalid ack bytes",
			ack:      channeltypes.NewResultAcknowledgement([]byte("test")),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "cannot unmarshall ICA acknowledgement",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			resp := &gammbalancer.MsgCreateBalancerPoolResponse{}
			err := keeper.ParseAck(tc.ack, tc.req, resp)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.Equal(t, tc.resp, resp)
		})
	}
}

func TestHandleIcaAcknowledgement(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	makeIcaPacket := _makeIcaPacket(t, setup.Cdc)

	var called bool
	testCases := []struct {
		name      string
		seq       uint64
		icaPacket icatypes.InterchainAccountPacketData
		ack       channeltypes.Acknowledgement
		setup     func()
		errorStr  string
	}{
		{
			name:      "valid",
			seq:       uint64(42),
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       makeAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
			setup: func() {
				called = false
				k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(sdk.Context, types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
					called = true
				})
			},
			errorStr: "",
		},
		{
			name:      "invalid acknowledgement",
			seq:       uint64(42),
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       channeltypes.NewResultAcknowledgement([]byte("test")),
			setup:     func() {},
			errorStr:  "cannot parse acknowledgement",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			tc.setup()
			err := k.HandleIcaAcknowledgement(ctx, tc.seq, tc.icaPacket, tc.ack)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.True(t, called)
		})
	}
}

func TestHandleIcaTimeout(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper

	seq := uint64(1)
	icaPacket := icatypes.InterchainAccountPacketData{}
	errorStr := "expected single message in packet"

	err := k.HandleIcaTimeout(ctx, seq, icaPacket)

	require.ErrorContains(t, err, errorStr)
}

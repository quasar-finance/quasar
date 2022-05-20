package keeper_test

import (
	"testing"

	"github.com/abag/quasarnode/testutil"
	"github.com/abag/quasarnode/x/intergamm/keeper"
	"github.com/abag/quasarnode/x/intergamm/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/stretchr/testify/require"
)

func makeIcaPacketPartial(t *testing.T, cdc codec.Codec) func(msg sdk.Msg) icatypes.InterchainAccountPacketData {
	return func(msg sdk.Msg) icatypes.InterchainAccountPacketData {
		data, err := icatypes.SerializeCosmosTx(cdc, []sdk.Msg{msg})
		require.NoError(t, err)

		return icatypes.InterchainAccountPacketData{
			Type: icatypes.EXECUTE_TX,
			Data: data,
		}
	}
}

func makeEmptyIcaPacketPartial(t *testing.T, cdc codec.Codec) func() icatypes.InterchainAccountPacketData {
	return func() icatypes.InterchainAccountPacketData {
		data, err := icatypes.SerializeCosmosTx(cdc, []sdk.Msg{})
		require.NoError(t, err)

		return icatypes.InterchainAccountPacketData{
			Type: icatypes.EXECUTE_TX,
			Data: data,
		}
	}
}

func makeInvalidIcaPacket() icatypes.InterchainAccountPacketData {
	return icatypes.InterchainAccountPacketData{
		Type: icatypes.EXECUTE_TX,
		Data: []byte("invalid"),
	}
}

func makeAckRawData(t *testing.T, req sdk.Msg, raw []byte) channeltypes.Acknowledgement {
	var msgData *sdk.TxMsgData

	if raw == nil {
		msgData = &sdk.TxMsgData{}
	} else {
		msgData = &sdk.TxMsgData{
			Data: []*sdk.MsgData{
				{
					MsgType: sdk.MsgTypeURL(req),
					Data:    raw,
				},
			},
		}
	}

	ackData, err := proto.Marshal(msgData)
	require.NoError(t, err)

	return channeltypes.NewResultAcknowledgement(ackData)
}

func makeAck(t *testing.T, req sdk.Msg, res proto.Message) channeltypes.Acknowledgement {
	resData, err := proto.Marshal(res)
	require.NoError(t, err)

	return makeAckRawData(t, req, resData)
}

func makeErrorAck(t *testing.T, errorStr string) channeltypes.Acknowledgement {
	return channeltypes.NewErrorAcknowledgement(errorStr)
}

func makeInvalidAck() channeltypes.Acknowledgement {
	return channeltypes.NewResultAcknowledgement([]byte("invalid"))
}

func TestParseAck(t *testing.T) {
	testCases := []struct {
		name     string
		ack      channeltypes.Acknowledgement
		req      *gammbalancer.MsgCreateBalancerPool
		resp     *gammbalancer.MsgCreateBalancerPoolResponse
		errorStr string
	}{
		{
			name:     "valid",
			ack:      makeAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "",
		},
		{
			name:     "invalid ack bytes",
			ack:      makeInvalidAck(),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "cannot unmarshall ICA acknowledgement",
		},
		{
			name:     "invalid ack message bytes",
			ack:      makeAckRawData(t, &gammbalancer.MsgCreateBalancerPool{}, []byte("invalid")),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "cannot unmarshall ICA acknowledgement",
		},
		{
			name:     "invalid ack no message",
			ack:      makeAckRawData(t, &gammbalancer.MsgCreateBalancerPool{}, nil),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "only single msg acks are supported",
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
	makeIcaPacket := makeIcaPacketPartial(t, setup.Cdc)
	makeEmptyIcaPacket := makeEmptyIcaPacketPartial(t, setup.Cdc)
	tstSeq := uint64(42)

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
			name:      "valid MsgTransfer",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&ibctransfertypes.MsgTransfer{}),
			ack:       makeAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgTransfer(func(c sdk.Context, e types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgTransfer with error ack",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&ibctransfertypes.MsgTransfer{}),
			ack:       makeErrorAck(t, "test error"),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgTransfer(func(c sdk.Context, e types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
					require.Equal(t, "test error", e.Error)
					require.True(t, e.HasError())
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgCreateBalancerPool",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       makeAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(c sdk.Context, e types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitPool",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitPool{}),
			ack:       makeAck(t, &gammtypes.MsgExitPool{}, &gammtypes.MsgExitPoolResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
				})
			},
			errorStr: "",
		},
		{
			name:      "invalid ica packet",
			seq:       tstSeq,
			icaPacket: makeInvalidIcaPacket(),
			ack:       makeAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "cannot deserialize packet data",
		},
		{
			name:      "empty ica packet",
			seq:       tstSeq,
			icaPacket: makeEmptyIcaPacket(),
			ack:       makeAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "expected single message in packet",
		},
		{
			name:      "invalid ack bytes",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       makeAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "cannot parse acknowledgement",
		},
		{
			name:      "unsupported packet type",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&qbanktypes.MsgRequestDeposit{}),
			ack:       makeAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "unsupported packet type",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			called = false
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

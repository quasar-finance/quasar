package keeper_test

import (
	"errors"
	"testing"

	"github.com/cosmos/cosmos-sdk/codec"
	sdk "github.com/cosmos/cosmos-sdk/types"
	icatypes "github.com/cosmos/ibc-go/v5/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v5/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v5/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	gammtypes "github.com/quasarlabs/quasarnode/osmosis/gamm/types"
	lockuptypes "github.com/quasarlabs/quasarnode/osmosis/lockup/types"
	"github.com/quasarlabs/quasarnode/testutil"
	"github.com/quasarlabs/quasarnode/x/intergamm/keeper"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
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

func makeIcaAckRawData(t *testing.T, req sdk.Msg, raw []byte) channeltypes.Acknowledgement {
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

func makeIbcAck() channeltypes.Acknowledgement {
	return channeltypes.NewResultAcknowledgement([]byte{byte(1)})
}

func makeIcaAck(t *testing.T, req sdk.Msg, res proto.Message) channeltypes.Acknowledgement {
	resData, err := proto.Marshal(res)
	require.NoError(t, err)

	return makeIcaAckRawData(t, req, resData)
}

func makeErrorAck(t *testing.T, errorStr string) channeltypes.Acknowledgement {
	return channeltypes.NewErrorAcknowledgement(errors.New(errorStr))
}

func makeInvalidAck() channeltypes.Acknowledgement {
	return channeltypes.NewResultAcknowledgement([]byte("invalid"))
}

func TestParseIcaAck(t *testing.T) {
	testCases := []struct {
		name     string
		ack      channeltypes.Acknowledgement
		req      *gammbalancer.MsgCreateBalancerPool
		resp     *gammbalancer.MsgCreateBalancerPoolResponse
		errorStr string
	}{
		{
			name:     "valid",
			ack:      makeIcaAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
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
			ack:      makeIcaAckRawData(t, &gammbalancer.MsgCreateBalancerPool{}, []byte("invalid")),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "cannot unmarshall ICA acknowledgement",
		},
		{
			name:     "invalid ack no message",
			ack:      makeIcaAckRawData(t, &gammbalancer.MsgCreateBalancerPool{}, nil),
			req:      &gammbalancer.MsgCreateBalancerPool{},
			resp:     &gammbalancer.MsgCreateBalancerPoolResponse{},
			errorStr: "only single msg acks are supported",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			resp := &gammbalancer.MsgCreateBalancerPoolResponse{}
			err := keeper.ParseIcaAck(tc.ack, tc.req, resp)

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
	tstChan := "tstChan"
	tstPort := "tstPort"

	var called bool
	testCases := []struct {
		name      string
		seq       uint64
		channel   string
		portId    string
		icaPacket icatypes.InterchainAccountPacketData
		ack       channeltypes.Acknowledgement
		setup     func()
		errorStr  string
	}{
		{
			name:      "valid MsgCreateBalancerPool",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       makeIcaAck(t, &gammbalancer.MsgCreateBalancerPool{}, &gammbalancer.MsgCreateBalancerPoolResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgCreateBalancerPool(func(c sdk.Context, e types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid Ica MsgTransfer",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&ibctransfertypes.MsgTransfer{}),
			ack:       makeIcaAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup: func() {
				k.Hooks.IbcTransfer.AddHooksAckIcaIbcTransfer(func(c sdk.Context, e types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinPool",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinPool{}),
			ack:       makeIcaAck(t, &gammtypes.MsgJoinPool{}, &gammtypes.MsgJoinPoolResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgJoinPool(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitPool",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitPool{}),
			ack:       makeIcaAck(t, &gammtypes.MsgExitPool{}, &gammtypes.MsgExitPoolResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgExitPool(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinSwapExternAmountIn",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinSwapExternAmountIn{}),
			ack:       makeIcaAck(t, &gammtypes.MsgJoinSwapExternAmountIn{}, &gammtypes.MsgJoinSwapExternAmountInResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgJoinSwapExternAmountIn(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitSwapExternAmountOut",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitSwapExternAmountOut{}),
			ack:       makeIcaAck(t, &gammtypes.MsgExitSwapExternAmountOut{}, &gammtypes.MsgExitSwapExternAmountOutResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgExitSwapExternAmountOut(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinSwapShareAmountOut",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinSwapShareAmountOut{}),
			ack:       makeIcaAck(t, &gammtypes.MsgJoinSwapShareAmountOut{}, &gammtypes.MsgJoinSwapShareAmountOutResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgJoinSwapShareAmountOut(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitSwapShareAmountIn",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitSwapShareAmountIn{}),
			ack:       makeIcaAck(t, &gammtypes.MsgExitSwapShareAmountIn{}, &gammtypes.MsgExitSwapShareAmountInResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgExitSwapShareAmountIn(func(c sdk.Context, e types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgLockTokens",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&lockuptypes.MsgLockTokens{}),
			ack:       makeIcaAck(t, &lockuptypes.MsgLockTokens{}, &lockuptypes.MsgLockTokensResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgLockTokens(func(c sdk.Context, e types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgBeginUnlocking",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&lockuptypes.MsgBeginUnlocking{}),
			ack:       makeIcaAck(t, &lockuptypes.MsgBeginUnlocking{}, &lockuptypes.MsgBeginUnlockingResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgBeginUnlocking(func(c sdk.Context, e types.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "error in hook MsgLockTokens",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&lockuptypes.MsgLockTokens{}),
			ack:       makeIcaAck(t, &lockuptypes.MsgLockTokens{}, &lockuptypes.MsgLockTokensResponse{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksAckMsgLockTokens(func(c sdk.Context, e types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return errors.New("error")
				})
			},
			errorStr: "handling msg /osmosis.lockup.MsgLockTokens: acknowledgement hook failed",
		},
		{
			name:      "invalid ica packet",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeInvalidIcaPacket(),
			ack:       makeIcaAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "cannot deserialize packet data",
		},
		{
			name:      "empty ica packet",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeEmptyIcaPacket(),
			ack:       makeIcaAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "expected single message in packet",
		},
		{
			name:      "invalid ack bytes",
			seq:       tstSeq,
			channel:   tstChan,
			portId:    tstPort,
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			ack:       makeIcaAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:     func() {},
			errorStr:  "cannot parse acknowledgement",
		},
		{
			name:    "unsupported packet type",
			seq:     tstSeq,
			channel: tstChan,
			portId:  tstPort, icaPacket: makeIcaPacket(&qbanktypes.MsgRequestDeposit{}),
			ack:      makeIcaAck(t, &ibctransfertypes.MsgTransfer{}, &ibctransfertypes.MsgTransferResponse{}),
			setup:    func() {},
			errorStr: "unsupported packet type",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			called = false
			k.Hooks.Osmosis.ClearAckHooks()
			tc.setup()
			err := k.HandleIcaAcknowledgement(ctx, tc.seq, tc.channel, tc.portId, tc.icaPacket, tc.ack)

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

	makeIcaPacket := makeIcaPacketPartial(t, setup.Cdc)
	makeEmptyIcaPacket := makeEmptyIcaPacketPartial(t, setup.Cdc)
	tstSeq := uint64(42)

	var called bool
	testCases := []struct {
		name      string
		seq       uint64
		icaPacket icatypes.InterchainAccountPacketData
		setup     func()
		errorStr  string
	}{
		{
			name:      "valid Ica MsgTransfer",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&ibctransfertypes.MsgTransfer{}),
			setup: func() {
				k.Hooks.IbcTransfer.AddHooksTimeoutIcaIbcTransfer(func(c sdk.Context, e types.TimeoutExchange[*ibctransfertypes.MsgTransfer]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgCreateBalancerPool",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammbalancer.MsgCreateBalancerPool{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgCreateBalancerPool(func(c sdk.Context, e types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinPool",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinPool{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgJoinPool(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgJoinPool]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitPool",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitPool{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgExitPool(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgExitPool]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinSwapExternAmountIn",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinSwapExternAmountIn{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapExternAmountIn(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitSwapExternAmountOut",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitSwapExternAmountOut{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapExternAmountOut(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgJoinSwapShareAmountOut",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgJoinSwapShareAmountOut{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgJoinSwapShareAmountOut(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgExitSwapShareAmountIn",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&gammtypes.MsgExitSwapShareAmountIn{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgExitSwapShareAmountIn(func(c sdk.Context, e types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgLockTokens",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&lockuptypes.MsgLockTokens{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgLockTokens(func(c sdk.Context, e types.TimeoutExchange[*lockuptypes.MsgLockTokens]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "valid MsgBeginUnlocking",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&lockuptypes.MsgBeginUnlocking{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgBeginUnlocking(func(c sdk.Context, e types.TimeoutExchange[*lockuptypes.MsgBeginUnlocking]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:      "error in hook MsgLockTokens",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&lockuptypes.MsgLockTokens{}),
			setup: func() {
				k.Hooks.Osmosis.AddHooksTimeoutMsgLockTokens(func(c sdk.Context, e types.TimeoutExchange[*lockuptypes.MsgLockTokens]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)

					return errors.New("error")
				})
			},
			errorStr: "handling msg /osmosis.lockup.MsgLockTokens: timeout hook failed",
		},
		{
			name:      "invalid ica packet",
			seq:       tstSeq,
			icaPacket: makeInvalidIcaPacket(),
			setup:     func() {},
			errorStr:  "cannot deserialize packet data",
		},
		{
			name:      "empty ica packet",
			seq:       tstSeq,
			icaPacket: makeEmptyIcaPacket(),
			setup:     func() {},
			errorStr:  "expected single message in packet",
		},
		{
			name:      "unsupported packet type",
			seq:       tstSeq,
			icaPacket: makeIcaPacket(&qbanktypes.MsgRequestDeposit{}),
			setup:     func() {},
			errorStr:  "unsupported packet type",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			called = false
			k.Hooks.Osmosis.ClearTimeoutHooks()
			tc.setup()
			err := k.HandleIcaTimeout(ctx, tc.seq, tc.icaPacket)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.True(t, called)
		})
	}
}

func TestHandleIbcTransferAcknowledgement(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	tstSeq := uint64(42)
	tstDenom := "testDenom"

	var called bool
	testCases := []struct {
		name           string
		seq            uint64
		transferPacket ibctransfertypes.FungibleTokenPacketData
		ack            channeltypes.Acknowledgement
		setup          func()
		errorStr       string
	}{
		{
			name: "valid FungibleTokenPacketData",
			seq:  tstSeq,
			transferPacket: ibctransfertypes.FungibleTokenPacketData{
				Denom: tstDenom,
			},
			ack: makeIbcAck(),
			setup: func() {
				k.Hooks.IbcTransfer.AddHooksAckIbcTransfer(func(c sdk.Context, e types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
					require.Equal(t, tstDenom, e.Request.Denom)

					return nil
				})
			},
			errorStr: "",
		},
		{
			name:           "valid FungibleTokenPacketData with error ack",
			seq:            tstSeq,
			transferPacket: ibctransfertypes.FungibleTokenPacketData{},
			ack:            makeErrorAck(t, "test error"),
			setup: func() {
				k.Hooks.IbcTransfer.AddHooksAckIbcTransfer(func(c sdk.Context, e types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
					require.NotEmpty(t, e.Error)
					require.True(t, e.HasError())

					return nil
				})
			},
			errorStr: "",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			called = false
			k.Hooks.IbcTransfer.ClearAckHooks()
			tc.setup()
			err := k.HandleIbcTransferAcknowledgement(ctx, tc.seq, tc.transferPacket, tc.ack)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.True(t, called)
		})
	}
}

func TestHandleIbcTimeout(t *testing.T) {
	setup := testutil.NewTestSetup(t)
	ctx, k := setup.Ctx, setup.Keepers.InterGammKeeper
	tstSeq := uint64(42)
	tstDenom := "testDenom"

	var called bool
	testCases := []struct {
		name           string
		seq            uint64
		transferPacket ibctransfertypes.FungibleTokenPacketData
		setup          func()
		errorStr       string
	}{
		{
			name: "valid FungibleTokenPacketData",
			seq:  tstSeq,
			transferPacket: ibctransfertypes.FungibleTokenPacketData{
				Denom: tstDenom,
			},
			setup: func() {
				k.Hooks.IbcTransfer.AddHooksTimeoutIbcTransfer(func(c sdk.Context, e types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]) error {
					called = true
					require.Equal(t, tstSeq, e.Sequence)
					require.Equal(t, tstDenom, e.Request.Denom)

					return nil
				})
			},
			errorStr: "",
		},
	}
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			called = false
			k.Hooks.IbcTransfer.ClearTimeoutHooks()
			tc.setup()
			err := k.HandleIbcTransferTimeout(ctx, tc.seq, tc.transferPacket)

			if tc.errorStr != "" {
				require.ErrorContains(t, err, tc.errorStr)
				return
			}

			require.NoError(t, err)
			require.True(t, called)
		})
	}
}

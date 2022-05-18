package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
	gammbalancer "github.com/osmosis-labs/osmosis/v7/x/gamm/pool-models/balancer"
	gammtypes "github.com/osmosis-labs/osmosis/v7/x/gamm/types"
	"github.com/pkg/errors"
)

func (k *Keeper) HandleIcaAcknowledgement(
	ctx sdk.Context,
	sequence uint64,
	icaPacket icatypes.InterchainAccountPacketData,
	ack channeltypes.Acknowledgement,
) error {
	msgs, err := icatypes.DeserializeCosmosTx(k.cdc, icaPacket.GetData())
	if err != nil {
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "cannot deserialize packet data")
	}

	if len(msgs) != 1 {
		return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "expected single message in packet")
	}

	msg := msgs[0]
	switch req := msg.(type) {
	case *ibctransfertypes.MsgTransfer:
		resp := &ibctransfertypes.MsgTransferResponse{}
		err := parseAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgTransfer {
			h(ctx, ex)
		}

	case *gammbalancer.MsgCreateBalancerPool:
		resp := &gammbalancer.MsgCreateBalancerPoolResponse{}
		err := parseAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammbalancer.MsgCreateBalancerPool, *gammbalancer.MsgCreateBalancerPoolResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgCreateBalancerPool {
			h(ctx, ex)
		}

	case *gammtypes.MsgJoinPool:
		resp := &gammtypes.MsgJoinPoolResponse{}
		err := parseAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgJoinPool, *gammtypes.MsgJoinPoolResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgJoinPool {
			h(ctx, ex)
		}

	case *gammtypes.MsgExitPool:
		resp := &gammtypes.MsgExitPoolResponse{}
		err := parseAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgExitPool, *gammtypes.MsgExitPoolResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgExitPool {
			h(ctx, ex)
		}

	default:
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "unsupported packet type")
	}

	return nil
}

func (k *Keeper) HandleIcaTimeout(
	ctx sdk.Context,
	sequence uint64,
	icaPacket icatypes.InterchainAccountPacketData,
) error {
	msgs, err := icatypes.DeserializeCosmosTx(k.cdc, icaPacket.GetData())
	if err != nil {
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "cannot deserialize packet data")
	}

	if len(msgs) != 1 {
		return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "expected single message in packet")
	}

	msg := msgs[0]
	switch req := msg.(type) {
	case *ibctransfertypes.MsgTransfer:
		ex := types.TimeoutExchange[*ibctransfertypes.MsgTransfer]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgTransfer {
			h(ctx, ex)
		}

	case *gammbalancer.MsgCreateBalancerPool:
		ex := types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgCreateBalancerPool {
			h(ctx, ex)
		}

	case *gammtypes.MsgJoinPool:
		ex := types.TimeoutExchange[*gammtypes.MsgJoinPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgJoinPool {
			h(ctx, ex)
		}

	case *gammtypes.MsgExitPool:
		ex := types.TimeoutExchange[*gammtypes.MsgExitPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgExitPool {
			h(ctx, ex)
		}

	default:
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "unsupported packet type")
	}

	return nil
}

// Spec doc:
// https://github.com/cosmos/ibc-go/blob/main/docs/apps/interchain-accounts/auth-modules.md#onacknowledgementpacket
func parseAck(ack channeltypes.Acknowledgement, request sdk.Msg, response proto.Message) error {
	if ack.GetError() != "" {
		return nil
	}

	txMsgData := &sdk.TxMsgData{}
	if err := proto.Unmarshal(ack.GetResult(), txMsgData); err != nil {
		return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
	}

	switch len(txMsgData.Data) {
	case 0:
		// see documentation below for SDK 0.46.x or greater
		return errors.New("currently unsupported operation")
	default:
		if len(txMsgData.Data) != 1 {
			return errors.New("only single msg acks are supported")
		}

		msgData := txMsgData.Data[0]
		msgType := msgData.GetMsgType()

		if msgType != sdk.MsgTypeURL(request) {
			return errors.New("ack response does not match request")
		}

		err := proto.Unmarshal(msgData.Data, response)
		if err != nil {
			return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
		}

		return nil
	}
}

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
	lockuptypes "github.com/osmosis-labs/osmosis/v7/x/lockup/types"
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
	case *gammbalancer.MsgCreateBalancerPool:
		resp := &gammbalancer.MsgCreateBalancerPoolResponse{}
		err := ParseIcaAck(ack, req, resp)
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
		err := ParseIcaAck(ack, req, resp)
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

	case *gammtypes.MsgJoinSwapExternAmountIn:
		resp := &gammtypes.MsgJoinSwapExternAmountInResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgJoinSwapExternAmountIn, *gammtypes.MsgJoinSwapExternAmountInResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgJoinPoolSingleDenom {
			h(ctx, ex)
		}

	case *gammtypes.MsgExitPool:
		resp := &gammtypes.MsgExitPoolResponse{}
		err := ParseIcaAck(ack, req, resp)
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

	case *lockuptypes.MsgLockTokens:
		resp := &lockuptypes.MsgLockTokensResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*lockuptypes.MsgLockTokens, *lockuptypes.MsgLockTokensResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgLockTokens {
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

	case *gammtypes.MsgJoinSwapExternAmountIn:
		ex := types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgJoinPoolSingleDenom {
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

	case *lockuptypes.MsgLockTokens:
		ex := types.TimeoutExchange[*lockuptypes.MsgLockTokens]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgLockTokens {
			h(ctx, ex)
		}

	default:
		return sdkerrors.Wrap(channeltypes.ErrInvalidPacket, "unsupported packet type")
	}

	return nil
}

func (k *Keeper) HandleIbcTransferAcknowledgement(
	ctx sdk.Context,
	sequence uint64,
	transferPacket ibctransfertypes.FungibleTokenPacketData,
	ack channeltypes.Acknowledgement,
) error {
	ex := types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]{
		Sequence: sequence,
		Error:    ack.GetError(),
		Request:  &transferPacket,
		Response: &types.MsgEmptyIbcResponse{},
	}
	for _, h := range k.Hooks.IbcTransfer.ackIbcTransfer {
		h(ctx, ex)
	}

	return nil
}

func (k *Keeper) HandleIbcTransferTimeout(
	ctx sdk.Context,
	sequence uint64,
	transferPacket ibctransfertypes.FungibleTokenPacketData,
) error {
	ex := types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]{
		Sequence: sequence,
		Request:  &transferPacket,
	}
	for _, h := range k.Hooks.IbcTransfer.timeoutIbcTransfer {
		h(ctx, ex)
	}

	return nil
}

// Spec doc:
// https://github.com/cosmos/ibc-go/blob/main/docs/apps/interchain-accounts/auth-modules.md#onacknowledgementpacket
func ParseIcaAck(ack channeltypes.Acknowledgement, request sdk.Msg, response proto.Message) error {
	var err error

	if ack.GetError() != "" {
		return nil
	}

	txMsgData := &sdk.TxMsgData{}
	err = proto.Unmarshal(ack.GetResult(), txMsgData)
	if err != nil {
		return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
	}

	// see documentation below for SDK 0.46.x or greater as Data will MsgData will be empty (new field added)
	if len(txMsgData.Data) != 1 {
		return errors.New("only single msg acks are supported")
	}

	msgData := txMsgData.Data[0]
	msgType := msgData.GetMsgType()

	if msgType != sdk.MsgTypeURL(request) {
		return errors.New("ack response does not match request")
	}

	err = proto.Unmarshal(msgData.GetData(), response)
	if err != nil {
		return errors.Wrap(err, "cannot unmarshall ICA acknowledgement")
	}

	return nil
}

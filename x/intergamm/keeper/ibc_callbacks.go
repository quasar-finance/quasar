package keeper

import (
	gammbalancer "github.com/abag/quasarnode/osmosis/v9/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/osmosis/v9/gamm/types"
	lockuptypes "github.com/abag/quasarnode/osmosis/v9/lockup/types"
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	icatypes "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	proto "github.com/gogo/protobuf/proto"
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
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.IbcTransfer.ackIcaIbcTransfer {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
		}

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
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
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
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
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
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
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
		for _, h := range k.Hooks.Osmosis.ackMsgJoinSwapExternAmountIn {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgExitSwapExternAmountOut:
		resp := &gammtypes.MsgExitSwapExternAmountOutResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgExitSwapExternAmountOut, *gammtypes.MsgExitSwapExternAmountOutResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgExitSwapExternAmountOut {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgJoinSwapShareAmountOut:
		resp := &gammtypes.MsgJoinSwapShareAmountOutResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgJoinSwapShareAmountOut, *gammtypes.MsgJoinSwapShareAmountOutResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgJoinSwapShareAmountOut {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgExitSwapShareAmountIn:
		resp := &gammtypes.MsgExitSwapShareAmountInResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*gammtypes.MsgExitSwapShareAmountIn, *gammtypes.MsgExitSwapShareAmountInResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgExitSwapShareAmountIn {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
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
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *lockuptypes.MsgBeginUnlocking:
		resp := &lockuptypes.MsgBeginUnlockingResponse{}
		err := ParseIcaAck(ack, req, resp)
		if err != nil {
			return sdkerrors.Wrap(channeltypes.ErrInvalidAcknowledgement, "cannot parse acknowledgement")
		}
		ex := types.AckExchange[*lockuptypes.MsgBeginUnlocking, *lockuptypes.MsgBeginUnlockingResponse]{
			Sequence: sequence,
			Error:    ack.GetError(),
			Request:  req,
			Response: resp,
		}
		for _, h := range k.Hooks.Osmosis.ackMsgBeginUnlocking {
			if err := h(ctx, ex); err != nil {
				return types.NewErrAcknowledgementHookFailed(sdk.MsgTypeURL(req))
			}
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
		for _, h := range k.Hooks.IbcTransfer.timeoutIcaIbcTransfer {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammbalancer.MsgCreateBalancerPool:
		ex := types.TimeoutExchange[*gammbalancer.MsgCreateBalancerPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgCreateBalancerPool {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgJoinPool:
		ex := types.TimeoutExchange[*gammtypes.MsgJoinPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgJoinPool {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgExitPool:
		ex := types.TimeoutExchange[*gammtypes.MsgExitPool]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgExitPool {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgJoinSwapExternAmountIn:
		ex := types.TimeoutExchange[*gammtypes.MsgJoinSwapExternAmountIn]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgJoinSwapExternAmountIn {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgExitSwapExternAmountOut:
		ex := types.TimeoutExchange[*gammtypes.MsgExitSwapExternAmountOut]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgExitSwapExternAmountOut {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgJoinSwapShareAmountOut:
		ex := types.TimeoutExchange[*gammtypes.MsgJoinSwapShareAmountOut]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgJoinSwapShareAmountOut {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *gammtypes.MsgExitSwapShareAmountIn:
		ex := types.TimeoutExchange[*gammtypes.MsgExitSwapShareAmountIn]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgExitSwapShareAmountIn {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *lockuptypes.MsgLockTokens:
		ex := types.TimeoutExchange[*lockuptypes.MsgLockTokens]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgLockTokens {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
		}

	case *lockuptypes.MsgBeginUnlocking:
		ex := types.TimeoutExchange[*lockuptypes.MsgBeginUnlocking]{
			Sequence: sequence,
			Request:  req,
		}
		for _, h := range k.Hooks.Osmosis.timeoutMsgBeginUnlocking {
			if err := h(ctx, ex); err != nil {
				return types.NewErrTimeoutHookFailed(sdk.MsgTypeURL(req))
			}
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
		if err := h(ctx, ex); err != nil {
			return types.NewErrAcknowledgementHookFailed("FungibleTokenPacketData")
		}
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
		if err := h(ctx, ex); err != nil {
			return types.NewErrTimeoutHookFailed("FungibleTokenPacketData")
		}
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

package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

type IbcTransferHooks struct {
	ackIbcTransfer        []func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error
	ackIcaIbcTransfer     []func(sdk.Context, types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) error
	timeoutIbcTransfer    []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]) error
	timeoutIcaIbcTransfer []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.MsgTransfer]) error
}

func (ih *IbcTransferHooks) ClearAckHooks() {
	ih.ackIbcTransfer = nil
	ih.ackIcaIbcTransfer = nil
}

func (ih *IbcTransferHooks) ClearTimeoutHooks() {
	ih.timeoutIbcTransfer = nil
	ih.timeoutIcaIbcTransfer = nil
}

func (ih *IbcTransferHooks) AddHooksAckIbcTransfer(hs ...func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error) {
	ih.ackIbcTransfer = append(ih.ackIbcTransfer, hs...)
}

func (ih *IbcTransferHooks) AddHooksAckIcaIbcTransfer(hs ...func(sdk.Context, types.AckExchange[*ibctransfertypes.MsgTransfer, *ibctransfertypes.MsgTransferResponse]) error) {
	ih.ackIcaIbcTransfer = append(ih.ackIcaIbcTransfer, hs...)
}

func (ih *IbcTransferHooks) AddHooksTimeoutIbcTransfer(hs ...func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]) error) {
	ih.timeoutIbcTransfer = append(ih.timeoutIbcTransfer, hs...)
}

func (ih *IbcTransferHooks) AddHooksTimeoutIcaIbcTransfer(hs ...func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.MsgTransfer]) error) {
	ih.timeoutIcaIbcTransfer = append(ih.timeoutIcaIbcTransfer, hs...)
}

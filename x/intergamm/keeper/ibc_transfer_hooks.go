package keeper

import (
	"github.com/abag/quasarnode/x/intergamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"
)

type IbcTransferHooks struct {
	ackIbcTransfer     []func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error
	timeoutIbcTransfer []func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]) error
}

func (ih *IbcTransferHooks) ClearAckHooks() {
	ih.ackIbcTransfer = nil
}

func (ih *IbcTransferHooks) ClearTimeoutHooks() {
	ih.timeoutIbcTransfer = nil
}

func (ih *IbcTransferHooks) AddHooksAckIbcTransfer(hs ...func(sdk.Context, types.AckExchange[*ibctransfertypes.FungibleTokenPacketData, *types.MsgEmptyIbcResponse]) error) {
	ih.ackIbcTransfer = append(ih.ackIbcTransfer, hs...)
}

func (ih *IbcTransferHooks) AddHooksTimeoutIbcTransfer(hs ...func(sdk.Context, types.TimeoutExchange[*ibctransfertypes.FungibleTokenPacketData]) error) {
	ih.timeoutIbcTransfer = append(ih.timeoutIbcTransfer, hs...)
}

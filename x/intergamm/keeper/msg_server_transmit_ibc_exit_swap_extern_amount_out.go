package keeper

import (
	"context"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcExitSwapExternAmountOut(goCtx context.Context, out *types.MsgTransmitIbcExitSwapExternAmountOut) (*types.MsgTransmitIbcExitSwapExternAmountOutResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	seq, channel, portId, err := ms.k.TransmitIbcExitSwapExternAmountOut(ctx, out.GetCreator(), out.GetConnectionId(), out.TimeoutTimestamp, out.PoolId, out.GetTokenOutMins(), sdk.NewInt(out.GetShareInAmount()))
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcExitSwapExternAmountOutResponse{
		Seq: seq,
		Channel: channel,
		PortId: portId,
	}, nil
}

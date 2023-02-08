package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/intergamm/types"
)

func (ms msgServer) TransmitIbcJoinSwapExternAmountIn(goCtx context.Context, in *types.MsgTransmitIbcJoinSwapExternAmountIn) (*types.MsgTransmitIbcJoinSwapExternAmountInResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	seq, channel, portId, err := ms.k.TransmitIbcJoinSwapExternAmountIn(ctx, in.GetCreator(), in.GetConnectionId(), in.TimeoutTimestamp, in.PoolId, in.GetTokenIn(), sdk.NewInt(in.GetShareOutMinAmount()))
	if err != nil {
		return nil, err
	}
	return &types.MsgTransmitIbcJoinSwapExternAmountInResponse{
		Seq:     seq,
		Channel: channel,
		PortId:  portId,
	}, nil
}

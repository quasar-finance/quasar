package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) RequestWithdrawAll(goCtx context.Context, msg *types.MsgRequestWithdrawAll) (*types.MsgRequestWithdrawAllResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	// _ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdrawAll|%s\n", msg.String()))
	// TODO - Call orion vault to request withdraw all withdrwable amounts.

	return &types.MsgRequestWithdrawAllResponse{}, nil
}

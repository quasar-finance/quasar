package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"quasartodel/x/vestingcustom/types"
)

func (k msgServer) CreateVestingAccount(goCtx context.Context, msg *types.MsgCreateVestingAccount) (*types.MsgCreateVestingAccountResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx

	return &types.MsgCreateVestingAccountResponse{}, nil
}

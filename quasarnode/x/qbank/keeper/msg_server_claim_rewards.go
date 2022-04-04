package keeper

import (
	"context"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

//ClaimRewards Transfer the accumulated rewards to the depositors account.
// If types.MsgClaimRewards.VaultID is orion vault then claim will
// be processed from the orion vault.
// TODO | AUDIT | Claim reward function is to be called from the Orion vault.
func (k msgServer) ClaimRewards(goCtx context.Context, msg *types.MsgClaimRewards) (*types.MsgClaimRewardsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx
	// Call Orion vault module for claim reward.
	return &types.MsgClaimRewardsResponse{}, nil
}

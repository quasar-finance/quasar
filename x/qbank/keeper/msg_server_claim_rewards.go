package keeper

import (
	"context"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// ClaimRewards Transfer the accumulated rewards to the depositors account.
// If types.MsgClaimRewards.VaultID is orion vault then claim will
// be processed from the orion vault global reward account.
func (k msgServer) ClaimRewards(goCtx context.Context, msg *types.MsgClaimRewards) (*types.MsgClaimRewardsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)
	accAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}
	qcoins, found := k.GetUserClaimAmount(ctx, msg.Creator, msg.VaultID)
	if found {
		if msg.GetVaultID() == oriontypes.ModuleName {
			rewardAccName := oriontypes.CreateOrionRewardGloablMaccName()
			err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx, rewardAccName, accAddr, qcoins.Coins)
			if err != nil {
				panic(err)
			}
			k.ClaimAll(ctx, msg.Creator, msg.VaultID)
		}
	}

	// TODO - Define and Emit Events
	return &types.MsgClaimRewardsResponse{}, nil
}

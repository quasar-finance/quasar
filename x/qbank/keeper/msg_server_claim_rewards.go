package keeper

import (
	"context"

	sdk "github.com/cosmos/cosmos-sdk/types"
	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"
)

// ClaimRewards Transfer the accumulated rewards to the depositors account.
// If types.MsgClaimRewards.VaultID is orion vault then claim will
// be processed from the orion vault global reward account.
func (k msgServer) ClaimRewards(goCtx context.Context, msg *types.MsgClaimRewards) (*types.MsgClaimRewardsResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositor := msg.GetCreator()
	vaultId := msg.GetVaultID()

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	switch vaultId {
	case oriontypes.ModuleName:
		qcoins, found := k.GetUserClaimAmt(ctx, depositor, vaultId)
		if found {
			rewardAccName := oriontypes.CreateOrionRewardGloablMaccName()
			err := k.bankKeeper.SendCoinsFromModuleToAccount(
				ctx,
				rewardAccName,
				depositorAddr,
				qcoins.Coins,
			)
			if err != nil {
				return nil, err
			}

			k.ClaimAll(ctx, depositor, vaultId)
			k.AddUserClaimedRewards(ctx, depositor, vaultId, qcoins.Coins)
			ctx.EventManager().EmitEvent(
				types.CreateClaimRewardsEvent(ctx, depositorAddr, qcoins.Coins, vaultId),
			)
		}

	default:
		return nil, types.ErrInvalidVaultId
	}

	k.Logger(ctx).Info(
		"ClaimRewards",
		"Depositor", depositor,
		"VaultId", vaultId,
	)

	// TODO - Define and Emit Events
	return &types.MsgClaimRewardsResponse{}, nil
}

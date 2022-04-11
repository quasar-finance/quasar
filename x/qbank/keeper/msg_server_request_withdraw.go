package keeper

import (
	"context"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestWithdraw withdraw of previously deposited funds by the depositors from the
// requested vault. Vault will determine if the request can be processed further.
func (k msgServer) RequestWithdraw(goCtx context.Context, msg *types.MsgRequestWithdraw) (*types.MsgRequestWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositor := msg.GetCreator()
	coin := msg.GetCoin()
	vaultId := msg.GetVaultID()
	riskProfile := msg.GetRiskProfile()

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	switch vaultId {
	case oriontypes.ModuleName:
		wcoin := k.GetActualWithdrawableAmt(ctx, depositor, coin.Denom)
		if wcoin.Amount.LT(coin.Amount) {
			return nil, types.ErrWithdrawInsufficientFunds
		}

		// Transfer amount to depositor from vault module acc.
		err := k.bankKeeper.SendCoinsFromModuleToAccount(
			ctx,
			oriontypes.ModuleName,
			depositorAddr,
			sdk.NewCoins(coin),
		)
		if err != nil {
			return nil, err
		}

	default:
		return nil, types.ErrInvalidVaultId
	}

	k.Keeper.SubActualWithdrableAmt(ctx, depositor, coin)

	ctx.EventManager().EmitEvent(
		types.CreateWithdrawEvent(ctx, depositorAddr, coin, vaultId, riskProfile),
	)

	k.Logger(ctx).Info(
		"RequestWithdraw",
		"Depositor", depositor,
		"Coin", coin.String(),
		"VaultId", vaultId,
		"RiskProfile", riskProfile,
	)

	return &types.MsgRequestWithdrawResponse{}, nil
}

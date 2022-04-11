package keeper

import (
	"context"

	// orionmodulekeeper "github.com/abag/quasarnode/x/orion/keeper"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
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

	if msg.GetVaultID() == oriontypes.ModuleName {
		wcoin := k.GetActualWithdrawableAmt(ctx, depositor, coin.Denom)
		if wcoin.Amount.LT(msg.Coin.Amount) {
			return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "Requested amt is greater than withdrawable amt")
		}

		// Transfer amount to depositor from vault module acc.
		err := k.bankKeeper.SendCoinsFromModuleToAccount(
			ctx,
			oriontypes.ModuleName,
			depositorAddr,
			sdk.NewCoins(msg.GetCoin()),
		)
		if err != nil {
			return nil, err
		}
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

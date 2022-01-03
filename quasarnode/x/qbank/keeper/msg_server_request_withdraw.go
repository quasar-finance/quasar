package keeper

import (
	"context"
	"fmt"

	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) RequestWithdraw(goCtx context.Context, msg *types.MsgRequestWithdraw) (*types.MsgRequestWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// TODO: Handling the message
	_ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdraw|%s\n", msg.String()))

	// depositorAddr - User address who previously deposited
	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	withdraw := types.Withdraw{0, msg.GetRiskProfile(),
		msg.GetVaultID(), msg.GetCreator(),
		msg.GetAmount(), msg.GetDenom()}

	k.Keeper.AppendWithdraw(ctx, withdraw)

	amount, _ := sdk.NewIntFromString(msg.GetAmount())

	// Transfer amount to vault from depositor
	if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
		types.ModuleName, // TODO - msg.VaultID module
		depositorAddr,
		sdk.NewCoins(sdk.Coin{msg.GetDenom(), amount})); err != nil {
		return nil, err
	}

	// TODO - Burn receipt tokens. Ask vault to do so.

	// TODO - Position Management -

	// Subtracts the withdraw request amount.
	k.Keeper.SubUserDenomDeposit(ctx, msg.GetCreator(), sdk.Coin{msg.GetDenom(), amount})

	k.Logger(ctx).Info( //msg.GetCreator(),
		"RequestWithdraw|Withdraw|",
		"Depositor=", msg.GetCreator(),
		"Amount=", msg.GetAmount(),
		"Denom=", msg.GetDenom())

	return &types.MsgRequestWithdrawResponse{}, nil
}

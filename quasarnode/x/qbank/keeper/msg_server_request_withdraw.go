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

	// TODO - Request Vault to do Vault Logic.
	// As the vault is in design phase. Assuming vault has
	// successfully done its logic. and code is ready to do
	// do bank module and qbank module state transition.

	// Transfer amount to depositor from vault module acc.
	if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
		types.ModuleName, // TODO - msg.VaultID module
		depositorAddr,
		sdk.NewCoins(sdk.Coin{msg.GetDenom(), amount})); err != nil {
		return nil, err
	}

	// TODO - Burn receipt tokens. Ask vault to do so.

	// Subtracts the withdraw request amount to state transition the
	// user denom deposit kv store to reflect the current total deposit
	// after successful withdraw.
	k.Keeper.SubUserDenomDeposit(ctx, msg.GetCreator(), sdk.Coin{msg.GetDenom(), amount})

	k.Logger(ctx).Info( //msg.GetCreator(),
		"RequestWithdraw|Withdraw|",
		"Depositor=", msg.GetCreator(),
		"Amount=", msg.GetAmount(),
		"Denom=", msg.GetDenom())

	return &types.MsgRequestWithdrawResponse{}, nil
}

package keeper

import (
	"context"
	"fmt"

	// osmolpvmodulekeeper "github.com/abag/quasarnode/x/osmolpv/keeper"
	osmolpvypes "github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// @desc Function will request the withdraw of previously deposited funds by the depositors from the
// requested vault. Vault will determine if the request can be processed further.
// TODO - Early withdrawal from the Orion vault will cause early withdrawal fee.
func (k msgServer) RequestWithdraw(goCtx context.Context, msg *types.MsgRequestWithdraw) (*types.MsgRequestWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// _ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdraw|%s\n", msg.String()))

	// depositorAddr - User address who previously deposited
	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	// TODO - Withdrawal ID
	withdraw := types.Withdraw{0, msg.GetRiskProfile(),
		msg.GetVaultID(), msg.GetCreator(), msg.GetCoin()}

	k.Keeper.AppendWithdraw(ctx, withdraw)

	// amount, _ := sdk.NewIntFromString(msg.GetAmount())

	// TODO - Request Vault to do Vault Logic.
	// As the vault is in design phase. Assuming vault has
	// successfully done its logic. and code is ready to do
	// do bank module and qbank module state transition.

	// TODO - Withdraw to unstake type conversion
	if msg.GetVaultID() == osmolpvypes.ModuleName {

		// TODO - Calculate or Fetch total withdrawable amount
		// Maintain withdrawable amount based on the lockup periods.
		// If a user has locked up 1000atom for 7 days on Jan 1, then he
		// can not withdraw till Jan 7 same time.

		// qbank does not need to talk to the vault module to do this.
		// We have a pull based model for the orion vault; orion vault will pull data
		// from the qbank either at the end blocker or begin blocker.
		// err = k.oionKeeper.RequestWithdraw(ctx, msg.GetCreator(), msg.GetCoin())
		if err != nil {
			return nil, err
		}

	}

	// Transfer amount to depositor from vault module acc.
	if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
		osmolpvypes.ModuleName, // TODO - msg.VaultID module
		depositorAddr,
		sdk.NewCoins(withdraw.GetCoin())); err != nil {
		//sdk.NewCoins(sdk.Coin{msg.GetDenom(), amount})); err != nil {
		return nil, err
	}

	// TODO - Burn receipt tokens. Ask vault to do so.

	// Subtracts the withdraw request amount to state transition the user denom deposit kv store
	//to reflect the current total deposit after successful withdraw.
	k.Keeper.SubUserDenomDeposit(ctx, msg.GetCreator(), withdraw.GetCoin())
	k.Keeper.SubUserDeposit(ctx, msg.GetCreator(), withdraw.GetCoin())

	k.Logger(ctx).Info(
		"RequestWithdraw|Withdraw|",
		"Depositor=", msg.GetCreator(),
		"Coin=", msg.GetCoin().String())

	return &types.MsgRequestWithdrawResponse{}, nil
}

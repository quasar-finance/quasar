package keeper

import (
	"context"
	"fmt"

	// osmolpvmodulekeeper "github.com/abag/quasarnode/x/osmolpv/keeper"
	osmolpvypes "github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// RequestWithdraw withdraw of previously deposited funds by the depositors from the
// requested vault. Vault will determine if the request can be processed further.
func (k msgServer) RequestWithdraw(goCtx context.Context, msg *types.MsgRequestWithdraw) (*types.MsgRequestWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// _ = ctx
	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdraw|%s\n", msg.String()))

	// Using zero value of ID. ID field is redundant for this request.
	withdraw := types.Withdraw{Id: 0, RiskProfile: msg.GetRiskProfile(),
		VaultID: msg.GetVaultID(), DepositorAccAddress: msg.GetCreator(), Coin: msg.GetCoin()}

	// This call should be removed.
	// k.Keeper.AppendWithdraw(ctx, withdraw)

	if msg.GetVaultID() == osmolpvypes.ModuleName {
		wcoin := k.GetWithdrawableAmt(ctx, msg.Creator, msg.Coin.Denom)
		if wcoin.Amount.LT(msg.Coin.Amount) {
			return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "Requested amt is greater than withwrable amt")
		}
		depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
		if err != nil {
			return nil, err
		}
		// Transfer amount to depositor from vault module acc.
		if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
			osmolpvypes.ModuleName,
			depositorAddr,
			sdk.NewCoins(withdraw.GetCoin())); err != nil {
			return nil, err
		}
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

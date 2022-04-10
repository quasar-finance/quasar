package keeper

import (
	"context"
	"fmt"

	// orionmodulekeeper "github.com/abag/quasarnode/x/orion/keeper"
	orionypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// RequestWithdraw withdraw of previously deposited funds by the depositors from the
// requested vault. Vault will determine if the request can be processed further.
func (k msgServer) RequestWithdraw(goCtx context.Context, msg *types.MsgRequestWithdraw) (*types.MsgRequestWithdrawResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdraw|%s\n", msg.String()))

	if msg.GetVaultID() == orionypes.ModuleName {
		wcoin := k.GetActualWithdrawableAmt(ctx, msg.Creator, msg.Coin.Denom)
		if wcoin.Amount.LT(msg.Coin.Amount) {
			return nil, sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "Requested amt is greater than withwrable amt")
		}
		depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
		if err != nil {
			return nil, err
		}
		// Transfer amount to depositor from vault module acc.
		if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
			orionypes.ModuleName,
			depositorAddr,
			sdk.NewCoins(msg.GetCoin())); err != nil {
			return nil, err
		}
	}

	k.Keeper.SubActualWithdrableAmt(ctx, msg.GetCreator(), msg.GetCoin())

	k.Logger(ctx).Info(
		"RequestWithdraw|Withdraw|",
		"Depositor=", msg.GetCreator(),
		"Coin=", msg.GetCoin().String())

	return &types.MsgRequestWithdrawResponse{}, nil
}

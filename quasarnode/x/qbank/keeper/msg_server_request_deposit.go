package keeper

import (
	"context"
	"fmt"

	osmolptypes "github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// _ = ctx
	// TODO - sdk.coin to be used in place of amount and denom string.

	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|%s\n", msg.String()))

	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	deposit := types.Deposit{0, msg.GetRiskProfile(),
		msg.GetVaultID(), msg.GetCreator(), msg.GetCoin()}
	k.Keeper.AppendDeposit(ctx, deposit)

	// Transfer amount to vault from depositor
	if err := k.bankKeeper.SendCoinsFromAccountToModule(ctx,
		depositorAddr,
		osmolptypes.ModuleName, // TODO - msg.VaultID module
		sdk.NewCoins(deposit.GetCoin())); err != nil {
		return nil, err
	}

	// TODO - Mint receipt tokens. Ask vault to do so.

	// TODO - Position Management -

	k.Keeper.AddUserDenomDeposit(ctx, msg.GetCreator(), deposit.GetCoin())
	k.Keeper.AddUserDeposit(ctx, msg.GetCreator(), deposit.GetCoin())

	// k.Keeper.AppendDeposit(ctx, deposit)
	k.Logger(ctx).Info( //msg.GetCreator(),
		"RequestDeposit|Deposited|",
		"Depositor=", msg.GetCreator(),
		"Coin=", msg.GetCoin().String())

	// TODO - Events And Telementry

	return &types.MsgRequestDepositResponse{}, nil
}

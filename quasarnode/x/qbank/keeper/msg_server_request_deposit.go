package keeper

import (
	"context"
	"fmt"

	osmolptypes "github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestDeposit process the deposit request transaction message and store in the KV store
// With appropriate key and value combinations so the store can be used efficiently.
func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// _ = ctx
	// TODO - sdk.coin to be used in place of amount and denom string.

	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|%s\n", msg.String()))

	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	deposit := types.Deposit{Id: 0, RiskProfile: msg.GetRiskProfile(),
		VaultID: msg.GetVaultID(), DepositorAccAddress: msg.GetCreator(),
		Coin: msg.GetCoin(), LockupPeriod: msg.GetLockupPeriod()}
	k.Keeper.AppendDeposit(ctx, deposit)

	// Transfer amount to vault from depositor
	if err := k.bankKeeper.SendCoinsFromAccountToModule(ctx,
		depositorAddr,
		//osmolptypes.ModuleName,
		osmolptypes.CreateOrionStakingMaccName(msg.GetLockupPeriod()),
		//osmolptypes.ModuleName, // TODO - msg.VaultID module. Lockupperiod based module account
		sdk.NewCoins(deposit.GetCoin())); err != nil {
		return nil, err
	}
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AfterBankTransfer|%s\n", msg.String()))

	// TODO - Mint receipt tokens. Ask vault to do so.

	// Position Management -
	// TODO | AUDIT | some of the less important KV store are to be removed for efficiency purposes

	k.Keeper.AddUserDenomDeposit(ctx, msg.GetCreator(), deposit.GetCoin())
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AddUserDenomDeposit|%s\n", msg.String()))

	k.Keeper.AddUserDeposit(ctx, msg.GetCreator(), deposit.GetCoin())
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AddUserDeposit|%s\n", msg.String()))
	// TODO - Get Current Epoch day and call below AddUserDenomEpochLockupDeposit
	k.Keeper.AddUserDenomEpochLockupDeposit(ctx, msg.GetCreator(), deposit.GetCoin(), uint64(ctx.BlockHeight()), deposit.GetLockupPeriod())

	k.Keeper.AddUserDenomLockupDeposit(ctx, msg.GetCreator(), deposit.GetCoin(), deposit.GetLockupPeriod())
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AddUserDenomLockupDeposit|%s\n", msg.String()))

	// TODO - consider a blockheight as epochday for now. Integrate epochmodule later.
	k.Keeper.AddEpochLockupUserDenomDeposit(ctx, msg.GetCreator(), deposit.GetCoin(), uint64(ctx.BlockHeight()), deposit.GetLockupPeriod())
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AddEpochLockupUserDenomDeposit|%s|blockheight = %d\n", msg.String(), uint64(ctx.BlockHeight())))
	k.Logger(ctx).Info(
		"RequestDeposit|Deposited|",
		"Depositor=", msg.GetCreator(),
		"Coin=", msg.GetCoin().String())

	// TODO - Events And Telementry

	return &types.MsgRequestDepositResponse{}, nil
}

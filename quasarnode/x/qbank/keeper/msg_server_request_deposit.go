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

	k.Logger(ctx).Debug(fmt.Sprintf("RequestDeposit|%s\n", msg.String()))

	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}

	deposit := types.Deposit{Id: 0, RiskProfile: msg.GetRiskProfile(),
		VaultID: msg.GetVaultID(), DepositorAccAddress: msg.GetCreator(),
		Coin: msg.GetCoin(), LockupPeriod: msg.GetLockupPeriod()}

	// Transfer amount to vault from depositor
	if err := k.bankKeeper.SendCoinsFromAccountToModule(ctx,
		depositorAddr,
		osmolptypes.CreateOrionStakingMaccName(msg.GetLockupPeriod()),
		sdk.NewCoins(deposit.GetCoin())); err != nil {
		return nil, err
	}

	k.Keeper.AddUserDenomDeposit(ctx, msg.GetCreator(), deposit.GetCoin())
	k.Keeper.AddUserDeposit(ctx, msg.GetCreator(), deposit.GetCoin())

	// AUDIT TODO - consider a blockheight as epochday for now. Integrate epochmodule later.
	k.Keeper.AddEpochLockupUserDenomDeposit(ctx, msg.GetCreator(), deposit.GetCoin(), uint64(ctx.BlockHeight()), deposit.GetLockupPeriod())
	k.Logger(ctx).Info(fmt.Sprintf("RequestDeposit|AddEpochLockupUserDenomDeposit|%s|blockheight = %d\n", msg.String(), uint64(ctx.BlockHeight())))

	// TODO - Events And Telementry

	return &types.MsgRequestDepositResponse{}, nil
}

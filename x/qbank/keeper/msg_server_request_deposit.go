package keeper

import (
	"context"
	"fmt"

	osmolptypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestDeposit process the deposit request transaction message and store in the KV store
// With appropriate key and value combinations so the store can be used efficiently.
func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	k.Logger(ctx).Debug(fmt.Sprintf("RequestDeposit|%s\n", msg.String()))

	depositorAccAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		// TODO wrap error for context
		return nil, err
	}

	// Transfer amount to vault from depositor
	err = k.bankKeeper.SendCoinsFromAccountToModule(
		ctx,
		depositorAccAddr,
		osmolptypes.CreateOrionStakingMaccName(msg.GetLockupPeriod()),
		sdk.NewCoins(msg.GetCoin()),
	)
	if err != nil {
		// TODO wrap error for context
		return nil, err
	}

	k.Keeper.AddUserDenomDeposit(ctx, msg.GetCreator(), msg.GetCoin())
	k.Keeper.AddUserDeposit(ctx, msg.GetCreator(), msg.GetCoin())
	// AUDIT TODO - consider a blockheight as epochday for now. Integrate epochmodule later.
	k.Keeper.AddEpochLockupUserDenomDeposit(ctx, msg.GetCreator(), msg.GetCoin(), uint64(ctx.BlockHeight()), msg.GetLockupPeriod())

	// TODO - Events And Telementry

	// TODO AG document logging convention
	k.Logger(ctx).Info(
		"RequestDeposit|Deposited|",
		"Depositor=", msg.GetCreator(),
		"Coin=", msg.GetCoin().String(),
		"Epoch=", uint64(ctx.BlockHeight()),
	)

	return &types.MsgRequestDepositResponse{}, nil
}

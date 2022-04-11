package keeper

import (
	"context"

	osmolptypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestDeposit process the deposit request transaction message and store in the KV store
// With appropriate key and value combinations so the store can be used efficiently.
func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositor := msg.GetCreator()
	coin := msg.GetCoin()
	lockupPeriod := msg.GetLockupPeriod()
	// TODO get current epoch
	currentEpoch := uint64(ctx.BlockHeight())

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	// Transfer amount to vault from depositor
	err = k.bankKeeper.SendCoinsFromAccountToModule(
		ctx,
		depositorAddr,
		osmolptypes.CreateOrionStakingMaccName(lockupPeriod),
		sdk.NewCoins(coin),
	)
	if err != nil {
		return nil, err
	}

	// TODO AG merge these 3 calls into a single public function in the keeper
	k.Keeper.AddUserDenomDeposit(ctx, depositor, coin)
	k.Keeper.AddUserDeposit(ctx, depositor, coin)
	k.Keeper.AddEpochLockupUserDenomDeposit(ctx, depositor, coin, currentEpoch, lockupPeriod)

	ctx.EventManager().EmitEvent(
		types.CreateDepositEvent(ctx, depositorAddr, coin, lockupPeriod, currentEpoch),
	)

	k.Logger(ctx).Info(
		"RequestDeposit",
		"Depositor", depositor,
		"Coin", coin.String(),
		"LockupPeriod", lockupPeriod.String(),
		"Epoch", currentEpoch,
	)

	return &types.MsgRequestDepositResponse{}, nil
}

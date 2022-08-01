package keeper

import (
	"context"

	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/quasarlabs/quasarnode/x/qbank/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestDeposit process the deposit request transaction message and store in the KV store
// With appropriate key and value combinations so the store can be used efficiently.
func (k msgServer) RequestDeposit(goCtx context.Context, msg *types.MsgRequestDeposit) (*types.MsgRequestDepositResponse, error) {

	ctx := sdk.UnwrapSDKContext(goCtx)

	if !k.Enabled(ctx) {
		return nil, types.ErrQbankNotEnabled
	}

	depositor := msg.GetCreator()
	coin := msg.GetCoin()
	lockupPeriod := msg.GetLockupPeriod()
	minDollarDepositValue := k.MinOrionEpochDenomDollarDeposit(ctx)
	stablePrice, found := k.qoracleKeeper.GetStablePrice(ctx, coin.Denom)
	if !found {
		return nil, types.ErrStablePriceNotAvailable
	}
	currentEpoch := uint64(k.EpochsKeeper.GetEpochInfo(ctx,
		k.OrionEpochIdentifier(ctx)).CurrentEpoch)

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	dollarDepositValue := coin.Amount.ToDec().Mul(stablePrice)
	if dollarDepositValue.LT(minDollarDepositValue) {
		k.Logger(ctx).Info(
			"RequestDeposit FAIL",
			"Depositor", depositor,
			"Coin", coin.String(),
			"LockupPeriod", lockupPeriod.String(),
			"Epoch", currentEpoch,
			"stablePrice", stablePrice,
			"minDollarDepositValue", minDollarDepositValue,
			"dollarDepositValue", dollarDepositValue,
		)
		return nil, types.ErrInsufficientDollarDepositValue
	}

	// Transfer amount to vault from depositor
	err = k.bankKeeper.SendCoinsFromAccountToModule(
		ctx,
		depositorAddr,
		oriontypes.ModuleName,
		//osmolptypes.CreateOrionStakingMaccName(lockupPeriod),
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
		"stablePrice", stablePrice,
		"minDollarDepositValue", minDollarDepositValue,
		"dollarDepositValue", dollarDepositValue,
	)

	k.OnDeposit(ctx, msg.VaultID, coin) // Callback to vault OnDeposit method

	return &types.MsgRequestDepositResponse{}, nil
}

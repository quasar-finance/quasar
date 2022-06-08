package keeper

import (
	"context"
	"github.com/cosmos/cosmos-sdk/store/prefix"

	oriontypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestWithdrawAll process the withdrawal transaction message for all denom withdraw in one transaction.
func (k msgServer) RequestWithdrawAll(goCtx context.Context, msg *types.MsgRequestWithdrawAll) (*types.MsgRequestWithdrawAllResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositor := msg.GetCreator()
	vaultId := msg.GetVaultID()

	depositorAddr, err := sdk.AccAddressFromBech32(depositor)
	if err != nil {
		return nil, err
	}

	switch vaultId {
	case oriontypes.ModuleName:
		store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ActualWithdrawableKeyKBP)
		// Iterate over types.ActualWithdrawableKeyKBP + {userAcc} + {":"}
		prefixKey := []byte(depositor)
		prefixKey = append(prefixKey, types.SepByte...)

		iter := sdk.KVStorePrefixIterator(store, prefixKey)
		defer iter.Close()

		k.Logger(ctx).Info(
			"GetEpochTotalActiveDeposits",
			"modulename", types.ModuleName,
			"blockheight", ctx.BlockHeight(),
			"prefixKey", string(prefixKey),
		)

		var coins sdk.Coins
		for ; iter.Valid(); iter.Next() {
			value := iter.Value()
			var coin sdk.Coin
			k.cdc.MustUnmarshal(value, &coin)
			coins = coins.Add(coin)
			k.EmptyActualWithdrawableAmt(ctx, depositor, coin.Denom)
		}

		err := k.bankKeeper.SendCoinsFromModuleToAccount(
			ctx,
			oriontypes.ModuleName,
			depositorAddr,
			coins,
		)
		if err != nil {
			return nil, err
		}

		k.AddTotalWithdrawAmt(ctx, depositor, vaultId, coins)

	default:
		return nil, types.ErrInvalidVaultId
	}

	ctx.EventManager().EmitEvent(
		types.CreateWithdrawAllEvent(ctx, depositorAddr, vaultId),
	)

	k.Logger(ctx).Info(
		"RequestWithdrawAll",
		"Depositor", depositor,
		"VaultId", vaultId,
	)

	return &types.MsgRequestWithdrawAllResponse{}, nil
}

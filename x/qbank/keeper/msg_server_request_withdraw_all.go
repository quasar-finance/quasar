package keeper

import (
	"context"
	"fmt"

	orionypes "github.com/abag/quasarnode/x/orion/types"
	"github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// RequestWithdrawAll process the withdraw transaction message for all denom withdraw in one transaction.
func (k msgServer) RequestWithdrawAll(goCtx context.Context, msg *types.MsgRequestWithdrawAll) (*types.MsgRequestWithdrawAllResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	depositorAddr, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return nil, err
	}
	k.Logger(ctx).Info(fmt.Sprintf("RequestWithdrawAll|%s\n", msg.String()))

	if msg.GetVaultID() == orionypes.ModuleName {
		// Iterate over types.ActualWithdrawableKeyKBP + {userAcc} + {":"}
		bytePrefix := types.ActualWithdrawableKeyKBP
		prefixKey := []byte(msg.Creator)
		prefixKey = append(bytePrefix, prefixKey...)
		prefixKey = append(prefixKey, types.SepByte...)

		store := ctx.KVStore(k.storeKey)
		iter := sdk.KVStorePrefixIterator(store, prefixKey)
		defer iter.Close()

		logger := k.Logger(ctx)
		logger.Info(fmt.Sprintf("GetEpochTotalActiveDeposits|modulename=%s|blockheight=%d|prefixKey=%s",
			types.ModuleName, ctx.BlockHeight(), string(prefixKey)))

		var coins sdk.Coins
		for ; iter.Valid(); iter.Next() {
			// key =  {denom}, value = sdk.Coin marshled
			value := iter.Value()
			var coin sdk.Coin
			k.cdc.MustUnmarshal(value, &coin)
			coins = coins.Add(coin)
			k.EmptyActualWithdrableAmt(ctx, msg.Creator, coin.Denom)
		}

		if err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx,
			orionypes.ModuleName,
			depositorAddr,
			coins); err != nil {
			return nil, err // AUDIT NOTE - Test it properly in the unit tests.
		}
	}

	return &types.MsgRequestWithdrawAllResponse{}, nil
}

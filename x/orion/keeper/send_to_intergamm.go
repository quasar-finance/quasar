package keeper

import (
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Intergamm module method wrappers
func (k Keeper) JoinPool(ctx sdk.Context, poolID uint64, shareOutAmount sdk.Int, tokenInMaxs []sdk.Coin) (uint64, error) {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|poolID=%v|shareOutAmount=%v|tokenInMaxs=%v\n",
		poolID, shareOutAmount, tokenInMaxs))

	owner := ""
	connectionId := ""
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()

	packetSeq, err := k.intergammKeeper.TransmitIbcJoinPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareOutAmount,
		tokenInMaxs,
	)

	return packetSeq, err
}

func (k Keeper) ExitPool(ctx sdk.Context, poolID uint64, shareInAmount sdk.Int, tokenOutMins []sdk.Coin) error {
	k.Logger(ctx).Info(fmt.Sprintf("Entered JoinPool|poolID=%v|shareInAmount=%v|tokenOutMins=%v\n",
		poolID, shareInAmount, tokenOutMins))

	owner := ""
	connectionId := ""
	timeoutTimestamp := time.Now().Add(time.Minute).Unix()

	_, err := k.intergammKeeper.TransmitIbcExitPool(
		ctx,
		owner,
		connectionId,
		uint64(timeoutTimestamp),
		poolID,
		shareInAmount,
		tokenOutMins,
	)

	return err
}

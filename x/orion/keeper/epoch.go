package keeper

import (
	"fmt"

	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	// TODO get epoch identifier from params
	logger := k.Logger(ctx)

	if epochIdentifier == "minute" {
		logger.Info("epoch ended", "identifier", epochIdentifier,
			"number", epochNumber,
			"blockheight", ctx.BlockHeight())

		// Logic :
		// 1. Get the list of meissa strategies registered.
		// 2. Join Pool Logic - Iteratively Execute the strategy code for each meissa sub strategy registered.
		// 3. Exit Pool Logic - Check the strategy code for Exit conditions And call Exit Pool.
		// 4. Withdraw Pool - Check the strategy code for withdraw condition and call withdraw
		// 5. Update Strategy Positions.

		// Assumption 1 minute is one epoch day for testing
		for lockupEnm, lockupStr := range qbanktypes.LockupTypes_name {

			logger.Debug(fmt.Sprintf("Orion AfterEpochEnd|epochday=%d|blockheight=%d|lockup=%v",
				epochNumber, ctx.BlockHeight(), lockupStr))

			if lockupStr != "Invalid" {
				lockupPeriod := qbanktypes.LockupTypes(lockupEnm)
				k.ExecuteMeissa(ctx, uint64(epochNumber), lockupPeriod)
			}
		}

	}
}

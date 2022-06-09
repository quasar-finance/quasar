package keeper

import (
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k Keeper) IsOrionICACreated(ctx sdk.Context) (string, bool) {
	return k.intergammKeeper.IsICARegistered(ctx, k.getConnectionId("osmosis"), k.getOwnerAccStr())
}

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	// TODO get epoch identifier from params
	// TODO review error handling of this function
	logger := k.Logger(ctx)
	var err error
	var icaFound bool
	var addr string
	// For testing purposes - Param should be used then.
	// Send tokens to destination chain.
	if epochIdentifier == "minute" { // TODO - config ibc transfer epoch identifier.

		if !k.Enabled(ctx) {
			return
		}

		addr, icaFound = k.IsOrionICACreated(ctx)
		if !icaFound {
			k.intergammKeeper.RegisterInterchainAccount(ctx, k.getConnectionId("osmosis"), k.getOwnerAccStr())
		} else {
			logger.Info("AfterEpochEnd", "Orion Interchain Account Found", addr)
		}

		logger.Info("AfterEpochEnd", "available fund", k.GetAvailableInterchainFund(ctx))

		// ei := k.epochsKeeper.GetEpochInfo(ctx, "day")
		ei := k.epochsKeeper.GetEpochInfo(ctx, k.LpEpochId(ctx))
		currEpochDay := ei.CurrentEpoch

		logger.Info("AfterEpochEnd", "minutes identifier", epochIdentifier,
			"number", epochNumber,
			"blockheight", ctx.BlockHeight(),
			"ei", ei)

		totalEpochDeposits := k.qbankKeeper.GetTotalEpochDeposits(ctx, uint64(currEpochDay))
		totalEpochTransferred := k.GetTotalEpochTransffered(ctx, uint64(currEpochDay))
		diffCoins := totalEpochDeposits.Sub(totalEpochTransferred)
		logger.Info("AfterEpochEnd",
			"totalEpochDeposits", totalEpochDeposits,
			"totalEpochTransferred", totalEpochTransferred,
			"diffCoins", diffCoins,
		)

		for _, c := range diffCoins {
			seqNo, err := k.IBCTokenTransfer(ctx, c)
			logger.Info("AfterEpochEnd",
				"seqNo", seqNo,
				"err", err,
				"coin", c,
			)
			logger.Info("AfterEpochEnd 2", "available fund", k.GetAvailableInterchainFund(ctx))

			// k.SetIBCTokenTransferRecord(ctx, seqNo, c)
		}

	}

	if epochIdentifier == k.LpEpochId(ctx) {
		logger.Info("epoch ended", "identifier", epochIdentifier,
			"number", epochNumber,
			"blockheight", ctx.BlockHeight())

		if k.Enabled(ctx) && icaFound {
			// Logic :
			// 1. Get the list of meissa strategies registered.
			// 2. Join Pool Logic - Iteratively Execute the strategy code for each meissa sub strategy registered.
			// 3. Exit Pool Logic - Check the strategy code for Exit conditions And call Exit Pool.
			// 4. Withdraw Pool - Check the strategy code for withdraw condition and call withdraw
			// 5. Update Strategy Positions.

			// Assumption 1 minute is one epoch day for testing
			for lockupEnm, lockupStr := range qbanktypes.LockupTypes_name {

				logger.Debug("Orion AfterEpochEnd", "epochday", epochNumber,
					"blockheight", ctx.BlockHeight(),
					"lockup", lockupStr)
				if lockupStr != "Invalid" {
					lockupPeriod := qbanktypes.LockupTypes(lockupEnm)
					err = k.ExecuteMeissa(ctx, uint64(epochNumber), lockupPeriod)
					if err != nil {
						panic(err)
					}
				}
			}

			// Refund distribution
			err = k.DistributeEpochLockupFunds(ctx, uint64(epochNumber))
			if err != nil {
				panic(err)
			}

			// Reward distribution
			_ = k.RewardDistribution(ctx, uint64(epochNumber))
			// TODO proper error handling for RewardDistribution once its issues are fixed
		} // k.Enabled(ctx)
	}
}

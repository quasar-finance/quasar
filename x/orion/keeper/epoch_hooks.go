package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	epochstypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
)

// EpochHooks wrapper struct
type EpochHooks struct {
	k Keeper
}

var _ epochstypes.EpochHooks = EpochHooks{}

// EpochHooks returns the wrapper struct.
func (k Keeper) EpochHooks() EpochHooks {
	return EpochHooks{k}
}

// epochs hooks
// Don't do anything pre epoch start.
func (h EpochHooks) BeforeEpochStart(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
}

func (h EpochHooks) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	h.k.AfterEpochEnd(ctx, epochIdentifier, epochNumber)
}

func (k Keeper) IsOrionICACreated(ctx sdk.Context) (string, bool) {
	c, found := k.GetConnectionId(ctx)
	//	c, found := k.GetConnectionId(ctx, "osmosis")

	if !found {
		return "", false
	}
	return k.intergammKeeper.IsICARegistered(ctx, c, k.getOwnerAccStr())
}

func (k Keeper) AfterEpochEnd(ctx sdk.Context, epochIdentifier string, epochNumber int64) {
	println("-------------------------------")
	println("AfterEpochEnd")
	// TODO get epoch identifier from params
	// TODO review error handling of this function
	logger := k.Logger(ctx)
	var err error
	var icaFound bool
	var addr string

	// TODO ->
	// Rewinding in case of emergency operation. It is possible that foundations decide to disable
	// Orion module temporarily and run emergency operations.

	if !k.Enabled(ctx) {
		return
	}

	println("epochIdentifier:  ", epochIdentifier)
	println("OwnerAddr:  ", k.getOwnerAccStr())

	// IBC Token Transfer.
	// For testing purposes - Param should be used then.
	// Send tokens to destination chain.
	if epochIdentifier == "minute" { // TODO - config ibc transfer epoch identifier.

		addr, icaFound = k.IsOrionICACreated(ctx)
		if !icaFound {
			// Print all connections ids to console
			logger.Info("AfterEpochEnd", "GetAllConnections", k.intergammKeeper.GetAllConnections(ctx))

			//			c, found := k.GetConnectionId(ctx, "osmosis")
			c, found := k.GetConnectionId(ctx)

			if !found {
				logger.Info("AfterEpochEnd", "GetConnectionId failed.")
			} else {
				println("ConnId:  ", c)
				println("OwnerAddr:  ", k.getOwnerAccStr())
				err := k.intergammKeeper.RegisterInterchainAccount(ctx, c, k.getOwnerAccStr())
				if err != nil {
					// panic(err)
					logger.Info("AfterEpochEnd", "RegisterInterchainAccount failed.", err)
				}
			}
			// return so we don't end up with calling token transfer logics, as token transfer is to be done
			// to orion ica account.
			return
		} else {
			logger.Info("AfterEpochEnd", "Orion Interchain Account Found", addr)
		}

		logger.Info("AfterEpochEnd", "available fund", k.GetAvailableInterchainFund(ctx))

		ei := k.epochsKeeper.GetEpochInfo(ctx, k.LpEpochId(ctx))
		currEpochDay := ei.CurrentEpoch

		logger.Info("AfterEpochEnd", "minutes identifier", epochIdentifier,
			"number", epochNumber,
			"BlockHeight", ctx.BlockHeight(),
			"ei", ei)

		totalEpochLockupCoinsDeposit := k.qbankKeeper.GetEpochLockupCoins(ctx, uint64(epochNumber))
		totalEpochLockupCoinsTransferred := k.GetTransferredEpochLockupCoins(ctx, uint64(epochNumber))

		println("totalEpochLockupCoinsDeposit: ", totalEpochLockupCoinsDeposit.String())
		println("totalEpochLockupCoinsTransferred: ", totalEpochLockupCoinsTransferred.String())

		denomDeposits := sdk.NewCoins()    // total deposited so far
		denomTransferred := sdk.NewCoins() // total transferred so far

		lockupDeposits := make(map[qbanktypes.LockupTypes]sdk.Coins)    // total a deposited for this lockup period
		lockupTransferred := make(map[qbanktypes.LockupTypes]sdk.Coins) // total a transferred for this lockup period

		//diffDenoms := sdk.NewCoins()
		diffLockups := make(map[qbanktypes.LockupTypes]sdk.Coins)

		for _, elcd := range totalEpochLockupCoinsDeposit.Infos {
			denomDeposits = denomDeposits.Add(elcd.GetCoin())

			if val, ok := lockupDeposits[elcd.LockupPeriod]; ok {
				lockupDeposits[elcd.LockupPeriod] = val.Add(elcd.Coin)
			} else {
				lockupDeposits[elcd.LockupPeriod] = sdk.NewCoins(elcd.Coin)
			}
		}
		println("denomDeposits: ", denomDeposits.String())
		for l, c := range lockupDeposits {
			println(l.String(), ":", c)
		}

		for _, elct := range totalEpochLockupCoinsTransferred.Infos {
			denomTransferred = denomTransferred.Add(elct.GetCoin())

			if val, ok := lockupTransferred[elct.LockupPeriod]; ok {
				lockupTransferred[elct.LockupPeriod] = val.Add(elct.Coin)
			} else {
				lockupTransferred[elct.LockupPeriod] = sdk.NewCoins(elct.Coin)
			}
		}
		println("denomTransferred: ", denomTransferred.String())
		for l, c := range lockupTransferred {
			println(l.String(), ":", c)
		}

		// diffDenoms = denomDeposits.Sub(denomTransferred)

		for l, c := range lockupDeposits {
			if v, ok := lockupTransferred[l]; ok {
				diffLockups[l] = c.Sub(v)
			} else {
				diffLockups[l] = c
			}
		}
		println("lockupDeposits")
		for l, c := range diffLockups {
			println(l.String(), ":", c)
		}

		// Now you need to process both the maps denomDeposits and lockupDeposits
		// Store them in a locka kv store.
		/*
			// Note - A separate send for each combination of <lockup, denom> should be done, to easily manage.
			// data structures. On ack fetch the EpochLockupCoinInfo from seq and add it to the
			// kv store corresponding to GetTransferredEpochLockupCoins
			for _, _ := range diffDenoms {
				// newly added coins
				// key -> sent1/epoch/seq/denom, value -> coin or value can be EpochLockupCoinInfo
				// Or <k,v > => <seqNo, EpochLockupCoinInfo>
			}
		*/

		for l, coins := range diffLockups {

			for _, c := range coins {
				seqNo, err := k.IBCTokenTransfer(ctx, c)
				logger.Info("AfterEpochEnd",
					"seqNo", seqNo,
					"err", err,
					"coin", c,
				)

				logger.Info("AfterEpochEnd 2", "available fund", k.GetAvailableInterchainFund(ctx))
				e := qbanktypes.EpochLockupCoinInfo{EpochDay: uint64(currEpochDay),
					LockupPeriod: l,
					Coin:         c}
				k.SetIBCTokenTransferRecord2(ctx, seqNo, e)
			}
		}

		/*
			totalEpochDeposits := k.qbankKeeper.GetTotalEpochDeposits(ctx, uint64(currEpochDay))
			totalEpochTransferred := k.GetTotalEpochTransferred(ctx, uint64(currEpochDay))
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
		*/

	}

	// Orion EOD activity.
	if epochIdentifier == k.LpEpochId(ctx) {
		logger.Info("epoch ended", "identifier", epochIdentifier,
			"number", epochNumber,
			"BlockHeight", ctx.BlockHeight())

		if k.Enabled(ctx) && icaFound {
			// Logic :
			// 1. Get the list of meissa strategies registered.
			// 2. Join Pool Logic - Iteratively Execute the strategy code for each meissa sub strategy registered.
			// 3. Exit Pool Logic - Check the strategy code for Exit conditions And call Exit Pool.
			// 4. Withdraw Pool - Check the strategy code for withdraw condition and call withdraw
			// 5. Update Strategy Positions.

			// Assumption 1 minute is one epoch day for testing
			////////////////////////////////////////////
			// Meissa Strategy Execution
			////////////////////////////////////////////
			for lockupEnm, lockupStr := range qbanktypes.LockupTypes_name {

				logger.Debug("Orion AfterEpochEnd", "epochNumber", epochNumber,
					"BlockHeight", ctx.BlockHeight(),
					"lockup", lockupStr)
				if lockupStr != "Invalid" {
					lockupPeriod := qbanktypes.LockupTypes(lockupEnm)
					err = k.ExecuteMeissa(ctx, uint64(epochNumber), lockupPeriod)
					if err != nil {
						panic(err)
					}
				}
			}

			////////////////////////////////////////////
			// Refund distribution
			////////////////////////////////////////////
			err = k.DistributeEpochLockupFunds(ctx, uint64(epochNumber))
			if err != nil {
				panic(err)
			}

			////////////////////////////////////////////
			// Reward distribution
			////////////////////////////////////////////
			err := k.RewardDistribution(ctx, uint64(epochNumber))
			if err != nil {
				panic(err)
			}

		} // k.Enabled(ctx)
	} // if epochIdentifier == k.LpEpochId(ctx)
}

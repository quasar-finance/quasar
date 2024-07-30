package app

import (
	storetypes "cosmossdk.io/store/types"
	"encoding/json"
	"log"

	sdk "github.com/cosmos/cosmos-sdk/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	servertypes "github.com/cosmos/cosmos-sdk/server/types"
	"github.com/cosmos/cosmos-sdk/x/staking"
)

// ExportAppStateAndValidators exports the state of the application for a genesis
// file.
func (app *QuasarApp) ExportAppStateAndValidators(
	forZeroHeight bool, jailAllowedAddrs []string,
) (servertypes.ExportedApp, error) {

	// as if they could withdraw from the start of the next block
	ctx := app.NewContext(true)

	// We export at last height + 1, because that's the height at which
	// Tendermint will start InitChain.
	height := app.LastBlockHeight() + 1
	if forZeroHeight {
		height = 0
		panic("forZeroHeight is set to true, but app.prepForZeroHeightGenesis is commented out")

		// app.prepForZeroHeightGenesis(ctx, jailAllowedAddrs)
	}

	genState, _ := app.mm.ExportGenesis(ctx, app.appCodec)
	appState, err := json.MarshalIndent(genState, "", "  ")
	if err != nil {
		return servertypes.ExportedApp{}, err
	}

	validators, err := staking.WriteValidators(ctx, app.StakingKeeper)
	if err != nil {
		return servertypes.ExportedApp{}, err
	}
	return servertypes.ExportedApp{
		AppState:        appState,
		Validators:      validators,
		Height:          height,
		ConsensusParams: app.BaseApp.GetConsensusParams(ctx),
	}, nil
}

// TODO SDK 50 - NOT SURE IF WE WANT TO FIX THIS,

// prepare for fresh start at zero height
// NOTE zero height genesis is a temporary feature which will be deprecated
//
//	in favour of export at a block height
func (app *QuasarApp) prepForZeroHeightGenesis(ctx sdk.Context, jailAllowedAddrs []string) {
	applyAllowedAddrs := false

	// check if there is a allowed address list
	if len(jailAllowedAddrs) > 0 {
		applyAllowedAddrs = true
	}

	allowedAddrsMap := make(map[string]bool)

	for _, addr := range jailAllowedAddrs {
		_, err := sdk.ValAddressFromBech32(addr)
		if err != nil {
			log.Fatal(err)
		}
		allowedAddrsMap[addr] = true
	}

	/* Just to be safe, assert the invariants on current state. */
	app.CrisisKeeper.AssertInvariants(ctx)

	/* Handle fee distribution state. */

	// withdraw all validator commission
	err := app.StakingKeeper.IterateValidators(ctx, func(_ int64, val stakingtypes.ValidatorI) (stop bool) {
		// todo valaddress
		_, err := app.DistrKeeper.WithdrawValidatorCommission(ctx, sdk.ValAddress(val.GetOperator()))
		if err != nil {
			panic(err)
		}
		return false
	})
	if err != nil {
		panic(err)
	}

	// withdraw all delegator rewards
	dels, err := app.StakingKeeper.GetAllDelegations(ctx)
	if err != nil {
		panic(err)
	}
	for _, delegation := range dels {
		// todo check accddress and valaddress
		_, err := app.DistrKeeper.WithdrawDelegationRewards(ctx, sdk.AccAddress(delegation.GetDelegatorAddr()), sdk.ValAddress(delegation.GetValidatorAddr()))
		if err != nil {
			panic(err)
		}
	}

	// clear validator slash events
	app.DistrKeeper.DeleteAllValidatorSlashEvents(ctx)

	// clear validator historical rewards
	app.DistrKeeper.DeleteAllValidatorHistoricalRewards(ctx)

	// set context height to zero
	height := ctx.BlockHeight()
	ctx = ctx.WithBlockHeight(0)

	// reinitialize all validators
	app.StakingKeeper.IterateValidators(ctx, func(_ int64, val stakingtypes.ValidatorI) (stop bool) {
		// donate any unwithdrawn outstanding reward fraction tokens to the community pool
		// todo check valaddress
		//scraps, err := app.DistrKeeper.GetValidatorOutstandingRewardsCoins(ctx, sdk.ValAddress(val.GetOperator()))
		//if err != nil {
		//	panic(err)
		//}

		// todo check this before v50
		//feePool := app.DistrKeeper.GEtFee
		//feePool.CommunityPool = feePool.CommunityPool.Add(scraps...)
		//app.DistrKeeper.SetFeePool(ctx, feePool)

		// todo check valaddress
		err = app.DistrKeeper.Hooks().AfterValidatorCreated(ctx, sdk.ValAddress(val.GetOperator()))
		if err != nil {
			log.Fatal(err)
		}
		return false
	})

	// reinitialize all delegations
	for _, del := range dels {
		// todo check valaddress and accaddress
		err := app.DistrKeeper.Hooks().BeforeDelegationCreated(ctx, sdk.AccAddress(del.GetDelegatorAddr()), sdk.ValAddress(del.GetValidatorAddr()))
		if err != nil {
			log.Fatal(err)
		}

		// todo check valaddress
		err = app.DistrKeeper.Hooks().AfterDelegationModified(ctx, sdk.AccAddress(del.GetDelegatorAddr()), sdk.ValAddress(del.GetValidatorAddr()))
		if err != nil {
			log.Fatal(err)
		}
	}

	// reset context height
	ctx = ctx.WithBlockHeight(height)

	/* Handle staking state. */

	// iterate through redelegations, reset creation height
	err = app.StakingKeeper.IterateRedelegations(ctx, func(_ int64, red stakingtypes.Redelegation) (stop bool) {
		for i := range red.Entries {
			red.Entries[i].CreationHeight = 0
		}
		app.StakingKeeper.SetRedelegation(ctx, red)
		return false
	})
	if err != nil {
		panic(err)
	}

	// iterate through unbonding delegations, reset creation height
	err = app.StakingKeeper.IterateUnbondingDelegations(ctx, func(_ int64, ubd stakingtypes.UnbondingDelegation) (stop bool) {
		for i := range ubd.Entries {
			ubd.Entries[i].CreationHeight = 0
		}
		app.StakingKeeper.SetUnbondingDelegation(ctx, ubd)
		return false
	})
	if err != nil {
		panic(err)
	}

	// Iterate through validators by power descending, reset bond heights, and
	// update bond intra-tx counters.
	keys := app.GetKVStoreKey()
	store := ctx.KVStore(keys[stakingtypes.StoreKey])
	iter := storetypes.KVStoreReversePrefixIterator(store, stakingtypes.ValidatorsKey)
	counter := int16(0)

	for ; iter.Valid(); iter.Next() {
		addr := sdk.ValAddress(iter.Key()[1:])
		validator, err := app.StakingKeeper.GetValidator(ctx, addr)
		if err != nil {
			panic("expected validator, not found")
		}

		validator.UnbondingHeight = 0
		if applyAllowedAddrs && !allowedAddrsMap[addr.String()] {
			validator.Jailed = true
		}

		err = app.StakingKeeper.SetValidator(ctx, validator)
		if err != nil {
			panic(err)
		}
		counter++
	}

	iter.Close()

	if _, err := app.StakingKeeper.ApplyAndReturnValidatorSetUpdates(ctx); err != nil {
		panic(err)
	}

	/* Handle slashing state. */

	// reset start height on signing infos
	err = app.SlashingKeeper.IterateValidatorSigningInfos(
		ctx,
		func(addr sdk.ConsAddress, info slashingtypes.ValidatorSigningInfo) (stop bool) {
			info.StartHeight = 0
			err := app.SlashingKeeper.SetValidatorSigningInfo(ctx, addr, info)
			if err != nil {
				return false
			}
			return false
		},
	)
	if err != nil {
		panic(err)
	}
}

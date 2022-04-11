package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// Note - Managing the amount to exit is the strategy concerns. But orion should make sure that users
// will get the same equivalent tokens of deposited tokens based on current market value.

// AddEpochExitAmt adds exited amount from the osmosis pools on a given epochday.
// Key - {types.ExitKBP} + {epochday} +  {":"} + {denom}
func (k Keeper) AddEpochExitAmt(ctx sdk.Context, epochday uint64, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochday, coin.Denom)

	k.Logger(ctx).Info(fmt.Sprintf("AddEpochExitAmt|key=%s|%s\n",
		string(key), coin))

	b := store.Get(key)
	if b == nil {
		value := k.cdc.MustMarshal(&coin)
		store.Set(key, value)
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Add(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// SubEpochExitAmt subs exited amount from the osmosis pools on a given epoch day
// Key -  {types.ExitKBP} + {epochday} +  {":"} + {denom}
func (k Keeper) SubEpochExitAmt(ctx sdk.Context, uid string, coin sdk.Coin, epochday uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochday, coin.Denom)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		panic(fmt.Errorf("exit amount is empty. Epoch: %v", epochday))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// GetEpochExitAmt returns the amount of exit positions on a given exit epoch day.
func (k Keeper) GetEpochExitAmt(ctx sdk.Context,
	epochday uint64, denom string) sdk.Coin {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochday, denom)
	b := store.Get(key)

	if b == nil {
		return sdk.NewCoin(denom, sdk.ZeroInt())
	}
	var coin sdk.Coin
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// SendCoinFromCollectionToAccount transfer balance from account to lockup reward account
// AUDIT  | Module account is the collection account for the deployed fund.
func (k Keeper) SendCoinFromCollectionToAccount(ctx sdk.Context, userAcc string, amt sdk.Coins) error {
	userAccAddr, _ := sdk.AccAddressFromBech32(userAcc)
	accName := types.ModuleName
	return k.bankKeeper.SendCoinsFromModuleToAccount(ctx, accName, userAccAddr, amt)
}

// DistributeEpochLockupFunds distribute the exited funds to the dipositors.
// Logic -
// 0. Fetch the actual deposit day and lockup periods corresponding to the todays distributionDay.
// 1. Calculate the total deposited funds on epochday for which today is the exit.
// 2. Validate how much funds are we able to exit from the osmosis today.
// 3. Check the difference between the exit funds and deposit funds for each denom.
// 		3.1 Mint necessary quasar for the end users.
//		3.2 OR Get it from the reserve.
//		3.3 Swap from other tokens available to us.
// AUDIT - TODO | Risk management for the multiple distribution calls need to be taken care.
func (k Keeper) DistributeEpochLockupFunds(ctx sdk.Context,
	distributionDay uint64) error {

	//  []types.DepositDayLockupPair
	ddlp := k.GetDepositDayInfos(ctx, distributionDay)
	epochUserInfo := k.ProcessDepositDayLockupPair(ctx, ddlp)

	// total denom and amounts deposited based on ddlp; for which today is the expectected exit day
	denomAmountMap := make(map[string]sdk.Int) // Used instead of sdk.Coins for efficiency
	for _, v := range epochUserInfo {
		if _, ok := denomAmountMap[v.Denom]; ok {
			denomAmountMap[v.Denom] = denomAmountMap[v.Denom].Add(v.Amt)
		} else {
			denomAmountMap[v.Denom] = v.Amt
		}
	}

	// denomRequiredAmtMap - map to indicate the required remaining amount that needs to be fulfilled
	// from treasury, or minting quasar or some other mechanism.
	denomRequiredAmtMap := make(map[string]sdk.Int)

	// denomExcessAmountMap - map to indicate the amount that is in excess of the denom amount deposited.
	// this amount will be used to transfer to the orion module treasury account.
	denomExcessAmountMap := make(map[string]sdk.Int)
	// denomAmountFromCollectionMap - map to indicate amount available from the collection during exit.
	denomAmountFromCollectionMap := make(map[string]sdk.Int)
	// denomAmountFromReserve - map to indicate the amount to be taken from the orion reserve.
	denomAmountFromReserve := make(map[string]sdk.Int)
	// denomOrions - map of denom and Orions amount allocated
	denomOrionsMap := make(map[string]sdk.Int)

	// Compare denomAmountMap and GetEpochExitAmt

	// For one denom - loop will run only one time
	for denom, amt := range denomAmountMap {
		c := k.GetEpochExitAmt(ctx, distributionDay, denom)
		if amt.LTE(c.Amount) {
			// All good, sufficient amount can be distributed back to users for this denom.
			// Transfer the remaining positive amount to the orion module treasury account.
			denomAmountFromCollectionMap[denom] = amt
			diff := c.Amount.Sub(amt) // ( collection amt - required amount ) => treasury
			if diff.IsPositive() {
				denomExcessAmountMap[denom] = diff
			}
		} else {
			// All the denom amt available from the collection is used
			denomAmountFromCollectionMap[denom] = c.Amount
			diff := amt.Sub(c.Amount)
			denomRequiredAmtMap[denom] = diff

			// Get the fund from othr resources -> treasury or mint
			r := k.GetReserveBalance(ctx, denom)
			if r.Amount.GTE(diff) {
				// All good, sufficient amount in the reserve. diff amount is to be taken from
				// the reserve balance.
				denomAmountFromReserve[denom] = diff

			} else {
				// All denom amount available in reserve will be used.
				denomRequiredAmtMap[denom] = denomRequiredAmtMap[denom].Sub(r.Amount)
				denomAmountFromReserve[denom] = r.Amount
				orions := k.MintAndAllocateOrions(ctx, sdk.NewCoin(denom, denomRequiredAmtMap[denom]))
				if orion, ok := denomAmountFromReserve[orions.Denom]; ok {
					denomAmountFromReserve[orions.Denom] = orion.Add(orions.Amount)
				} else {
					denomAmountFromReserve[orions.Denom] = orions.Amount
				}
				denomOrionsMap[denom] = orions.Amount
			}
		}

	}

	k.Logger(ctx).Info(
		fmt.Sprintf("DistributeEpochLockupFunds|Epochday=%v|denomRequiredAmtMap=%v|"+
			"denomExcessAmountMap=%v|denomOrionsMap=%v|denomAmountFromReserve=%v\n",
			distributionDay,
			denomRequiredAmtMap,
			denomExcessAmountMap,
			denomOrionsMap,
			denomAmountFromReserve))

	// AUDIT | TODO | Possible Optmization is to use InputOutput call from bank module
	// Process epochUserInfo, denomAmountMap, denomAmountFromCollectionMap, denomAmountFromReserve
	// Use Percentage Weight of a user denom from epochUserInfo
	for _, v := range epochUserInfo {
		c := denomAmountFromCollectionMap[v.Denom]
		r := denomAmountFromReserve[v.Denom]
		o := denomOrionsMap[v.Denom]
		var userCoins sdk.Coins
		if c.IsPositive() {
			userCoins = userCoins.Add(sdk.NewCoin(v.Denom, c.ToDec().Mul(v.Weight).TruncateInt()))
		}
		if r.IsPositive() {
			userCoins = userCoins.Add(sdk.NewCoin(v.Denom, r.ToDec().Mul(v.Weight).TruncateInt()))
		}
		if o.IsPositive() {
			userCoins = userCoins.Add(sdk.NewCoin(v.Denom, o.ToDec().Mul(v.Weight).TruncateInt()))
		}

		// Deduce management fee from UserAcc.
		var mgmtFees sdk.Coins
		for _, c := range userCoins {

			mgmtFees = mgmtFees.Add(k.CalcMgmtFee(ctx, c))
			k.qbankKeeper.AddActualWithdrableAmt(ctx, v.UserAcc, c)
		}
		userAccAddr, _ := sdk.AccAddressFromBech32(v.UserAcc)
		k.DeductAccFees(ctx, userAccAddr, types.MgmtFeeCollectorMaccName, mgmtFees)

	}
	return nil
}

// MintAndAllocateOrions is used for providing assurance to the end users that they will
// get their deposited equivalent tokens back irrespective of the IL loss in the deployed pools.
// Logic -
// 1. Mint  Equivalent amount of quasar and Mint Equivalent amount of Orions at current market price.
// 2. Lock the quasar token and use the orions to cover IL loss.
// 3. This way we don't loose Orions circulation from the system, and it can be used for network security
// to further enhance capital efficiency.
// Note - This way allocation of orions will be there only when we observe IL loss.
func (k Keeper) MintAndAllocateOrions(ctx sdk.Context, coin sdk.Coin) sdk.Coin {
	orions := k.CalcReceipts(ctx, coin)
	k.MintOrion(ctx, orions.Amount)
	qsr := k.CalcQSR(ctx, coin)
	// Note - As of now Mint in the orion module reserve acc . The QSR present in the orion module reserve
	// should not be used for the users distribution. They are considered as locked in
	// the module reserve account.
	k.bankKeeper.MintCoins(ctx, types.OrionReserveMaccName, sdk.NewCoins(qsr))
	return orions
}

// CalcQSR calculates the equivalent amount of quasar for the input sdk.coin
func (k Keeper) CalcQSR(ctx sdk.Context, coin sdk.Coin) sdk.Coin {
	p := k.GetQSRPrice(ctx, coin.Denom)
	amt := coin.Amount.ToDec().Mul(p).TruncateInt()
	return sdk.NewCoin("QSR", amt)
}

// GetQSRPrice gets the price of one denom in terms of QSR
// AUDIT | TODO
func (k Keeper) GetQSRPrice(ctx sdk.Context, denom string) sdk.Dec {
	return sdk.OneDec() // Assuming one denom = 1 QSR
}

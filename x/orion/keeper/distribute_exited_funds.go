package keeper

import (
	"fmt"

	"github.com/abag/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store/prefix"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

// AddEpochExitAmt adds exited denom amount collection from osmosis pools on a
// given epoch to the kv store
// Key - {types.ExitKBP} + {epochDay} +  {":"} + {denom}, Value = sdk.Coin
func (k Keeper) AddEpochExitAmt(ctx sdk.Context, epochDay uint64, coin sdk.Coin) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochDay, coin.Denom)

	k.Logger(ctx).Debug("AddEpochExitAmt", "key", string(key), "coin", coin)

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

// SubEpochExitAmt subs exited denom amount collection on a given epoch to the kv store
// Key -  {types.ExitKBP} + {epochDay} +  {":"} + {denom}, Value = sdk.Coin
func (k Keeper) SubEpochExitAmt(ctx sdk.Context, uid string, coin sdk.Coin, epochDay uint64) {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochDay, coin.Denom)
	b := store.Get(key)
	if b == nil {
		// Do nothing - Called by mistake.
		panic(fmt.Errorf("exit amount is empty. Epoch: %v", epochDay))
	} else {
		var storedCoin sdk.Coin
		k.cdc.MustUnmarshal(b, &storedCoin)
		storedCoin = storedCoin.Sub(coin)
		value := k.cdc.MustMarshal(&storedCoin)
		store.Set(key, value)
	}
}

// GetEpochExitAmt returns the denom amount of exited from on a given epoch day.
// Key -  {types.ExitKBP} + {epochDay} +  {":"} + {denom}, Value = sdk.Coin
func (k Keeper) GetEpochExitAmt(ctx sdk.Context,
	epochDay uint64, denom string) sdk.Coin {
	store := prefix.NewStore(ctx.KVStore(k.storeKey), types.ExitKBP)
	key := types.CreateEpochDenomKey(epochDay, denom)
	b := store.Get(key)

	if b == nil {
		return sdk.NewCoin(denom, sdk.ZeroInt())
	}
	var coin sdk.Coin
	k.cdc.MustUnmarshal(b, &coin)
	return coin
}

// GetCorrespondingEpochExitCoins calls GetEpochExitAmt for each denom in coins
// and returns all results as sdk.Coins
func (k Keeper) GetCorrespondingEpochExitCoins(
	ctx sdk.Context,
	epochDay uint64,
	coins sdk.Coins,
) (res sdk.Coins) {
	res = sdk.NewCoins()
	for _, coin := range coins {
		res = res.Add(k.GetEpochExitAmt(ctx, epochDay, coin.Denom))
	}
	return res
}

// SendCoinFromExitCollectionToAccount transfer tokens from orion module account to user account.
// Orion Module account is the exit collection account for the deployed fund.
func (k Keeper) SendCoinFromExitCollectionToAccount(ctx sdk.Context, userAcc string, amt sdk.Coins) error {
	userAccAddr, _ := sdk.AccAddressFromBech32(userAcc)
	accName := types.ModuleName
	return k.BankKeeper.SendCoinsFromModuleToAccount(ctx, accName, userAccAddr, amt)
}

// DistributeEpochLockupFunds distribute the exited funds to the depositors at the end of every epoch day.
// Logic -
// 0. Fetch the actual deposit day and lockup periods corresponding to the distributionDay.
// 1. Calculate the total deposited funds on actual deposit day (totalNeededCoins).
// 2. Calculate the amount of funds exited from the osmosis today (epochExitCoins).
// 3. Get the Balance of the reserve account (reserveCoins).
// 4. Compute the coins that should be allocated from epochExitCoins and/or reserveCoins, and total deficit.
// 5. Mint orions to cover the possible deficits.
// 6. Calculate share of each user and the fees.
// 7. Add the actual withdrawable amount in qbank kv store.
// 8. Deduct the fees.
func (k Keeper) DistributeEpochLockupFunds(
	ctx sdk.Context,
	distributionDay uint64,
) error {
	//  []types.DepositDayLockupPair
	ddlp := k.GetDepositDayInfos(ctx, distributionDay)
	epochUserInfo, totalNeededCoins := k.ProcessDepositDayLockupPair(ctx, ddlp)

	epochExitCoins := k.GetCorrespondingEpochExitCoins(ctx, distributionDay, totalNeededCoins)
	reserveCoins := k.GetAllReserveBalances(ctx)
	fromEpochExit, fromReserve, excessEpochExitCoins, totalDeficit :=
		CalculateCoinAllocations(totalNeededCoins, epochExitCoins, reserveCoins)
	availableCoins := fromEpochExit.Add(fromReserve...)

	orionsMintedForEachDenom, err := k.MintDeficit(ctx, totalDeficit)
	if err != nil {
		return err
	}

	k.Logger(ctx).Debug("DistributeEpochLockupFunds",
		"distributionDay", distributionDay,
		"totalNeededCoins", totalNeededCoins.String(),
		"excessEpochExitCoins", excessEpochExitCoins.String(),
		"orionsMintedForEachDenom", orionsMintedForEachDenom,
		"fromReserve", fromReserve.String())

	mgmtFeePercentage := k.MgmtFeePer(ctx)

	for _, v := range epochUserInfo {
		denom := v.Coin.Denom
		userCoins, mgmtFees := CalculateUserCoinsAndFees(denom, v.Weight, availableCoins, orionsMintedForEachDenom, mgmtFeePercentage)

		k.AddWithdrawableCoins(ctx, v.UserAcc, userCoins)

		userAccAddr, _ := sdk.AccAddressFromBech32(v.UserAcc)
		err := k.DeductAccFees(ctx, userAccAddr, types.MgmtFeeCollectorMaccName, mgmtFees)
		if err != nil {
			// TODO add error handling (probably non-fatal)
		}
	}
	return nil
}

// MintAndAllocateOrions is used for providing assurance to the end users that they will
// get their deposited equivalent tokens back irrespective of the IL loss in the deployed pools.
// Logic -
// 1. Mint  Equivalent amount of quasar and Mint Equivalent amount of Orions at current market price.
// 2. Lock the quasar token and use the orions to cover IL loss.
// 3. This way orion vault secure the orion receipt tokens using quasar which can be used for network security
// to further enhance capital efficiency [ Phase #2]
// Note - This way the actual allocation of orions is being done only when we observe IL loss.
func (k Keeper) MintAndAllocateOrions(ctx sdk.Context, coin sdk.Coin) (sdk.Coin, error) {
	orions, err := k.CalcReceipts(ctx, coin)
	if err != nil {
		return sdk.Coin{}, err
	}
	err = k.MintOrion(ctx, orions.Amount)
	if err != nil {
		return sdk.Coin{}, err
	}
	qsr, err := k.CalcQSR(ctx, coin)
	if err != nil {
		return sdk.Coin{}, err
	}
	// Note - As of now Mint in the orion module reserve acc . The QSR present in the orion module reserve
	// should not be used for distribution to the users. They are considered as locked in
	// the module reserve account.
	err = k.BankKeeper.MintCoins(ctx, types.OrionReserveMaccName, sdk.NewCoins(qsr))
	return orions, err
}

// CalcQSR calculates the equivalent amount of quasar for the input sdk.coin
func (k Keeper) CalcQSR(ctx sdk.Context, coin sdk.Coin) (sdk.Coin, error) {
	p, err := k.GetQSRPrice(ctx, coin.Denom)
	if err != nil {
		return sdk.Coin{}, err
	}
	amt := coin.Amount.ToDec().Mul(p).TruncateInt()
	return sdk.NewCoin("QSR", amt), nil
}

// GetQSRPrice gets the QSR price of one denom in terms of US dollar
func (k Keeper) GetQSRPrice(ctx sdk.Context, denom string) (sdk.Dec, error) {
	return k.GetRelativeStablePrice(ctx, denom, "QSR")
}

// MintDeficit mints (equivalent value of) orions to cover the deficits.
// Since all minted coins have the same denominations, a map is used (rather than sdk.Coins),
// to keep track of how much orions is minted for each denomination.
func (k Keeper) MintDeficit(ctx sdk.Context, totalDeficit sdk.Coins) (orionsMintedForEachDenom map[string]sdk.Coin, err error) {
	orionsMintedForEachDenom = make(map[string]sdk.Coin)
	for _, c := range totalDeficit {
		orionsMintedForEachDenom[c.Denom], err = k.MintAndAllocateOrions(ctx, c)
		if err != nil {
			return nil, err
		}
	}
	return orionsMintedForEachDenom, nil
}

// AddWithdrawableCoins iterates over coins, and call AddActualWithdrawableAmt of qbank for each coin.
func (k Keeper) AddWithdrawableCoins(ctx sdk.Context, addr string, coins sdk.Coins) {
	for _, coin := range coins {
		// TODO add similar function to QBank accepting sdk.Coins
		k.qbankKeeper.AddActualWithdrawableAmt(ctx, addr, coin)
	}
}

// CalculateCoinAllocations tries to allocate neededCoins first from epochExitCoins, and then from reserveCoins.
// It will also return the possible deficit.
// If epochExitCoins has more coins than the neededCoins, that will be returned too.
func CalculateCoinAllocations(
	neededCoins,
	epochExitCoins,
	reserveCoins sdk.Coins,
) (fromEpochExit, fromReserve, excessEpochExit, totalDeficit sdk.Coins) {
	fromEpochExit = neededCoins.Min(epochExitCoins)
	excessEpochExit = epochExitCoins.Sub(fromEpochExit)
	fromReserve = neededCoins.Sub(fromEpochExit).Min(reserveCoins)
	totalDeficit = neededCoins.Sub(fromEpochExit).Sub(fromReserve)
	return
}

// CalculateUserCoinsAndFees calculates the share of a user from the availableCoins anf minted orions,
// after deducting the management fees.
func CalculateUserCoinsAndFees(
	depositedDenom string,
	depositorWeight sdk.Dec,
	availableCoins sdk.Coins,
	orionsMintedForEachDenom map[string]sdk.Coin,
	mgmtFeePercentage sdk.Dec,
) (userCoins, mgmtFees sdk.Coins) {
	userDecCoins := sdk.NewDecCoins(sdk.NewDecCoin(depositedDenom, availableCoins.AmountOf(depositedDenom)))

	if mintedForThisDenom, exist := orionsMintedForEachDenom[depositedDenom]; exist {
		userDecCoins = userDecCoins.Add(sdk.NewDecCoinFromCoin(mintedForThisDenom))
	}

	userDecCoins = userDecCoins.MulDec(depositorWeight)

	mgmtFeesDecCoins := userDecCoins.MulDecTruncate(mgmtFeePercentage)
	userDecCoins = userDecCoins.Sub(mgmtFeesDecCoins)

	// TODO get the change into a public spending pool
	userCoins, _ /*userChangeCoins*/ = userDecCoins.TruncateDecimal()
	mgmtFees, _ /*mgmtFeeChangeCoins*/ = mgmtFeesDecCoins.TruncateDecimal()

	return
}

package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
)

func TestBankKeeper(tks *TestKeeperState, maccAddresses map[string]bool) (*authkeeper.AccountKeeper, sdk.Context) {
	storeKey := sdk.NewKVStoreKey(banktypes.StoreKey)
	tks.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, tks.TestDb)

	paramskeeper := tks.GetParamsKeeper()
	accountKeeper := tks.GetAccountKeeper()

	subspace := paramskeeper.Subspace(banktypes.ModuleName)
	bankKeeper := bankkeeper.NewBaseKeeper(
		tks.EncodingConfig.Marshaler, storeKey, accountKeeper, subspace, maccAddresses,
	)

	tks.BankKeeper = bankKeeper

	return &accountKeeper, tks.Ctx
}

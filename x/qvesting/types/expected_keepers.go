package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
)

// AccountKeeper defines the expected account keeper used for accounts operations
type AccountKeeper interface {
	GetAllAccounts(ctx sdk.Context) (accounts []authtypes.AccountI)
	GetAccount(ctx sdk.Context, addr sdk.AccAddress) authtypes.AccountI
	NewAccountWithAddress(ctx sdk.Context, addr sdk.AccAddress) authtypes.AccountI
	SetAccount(ctx sdk.Context, acc authtypes.AccountI)
}

// BankKeeper defines the expected account keeper used for bank operations
type BankKeeper interface {
	// Keeper
	IsSendEnabledCoins(ctx sdk.Context, coins ...sdk.Coin) error
	SendCoins(ctx sdk.Context, fromAddr sdk.AccAddress, toAddr sdk.AccAddress, amt sdk.Coins) error
	BlockedAddr(addr sdk.AccAddress) bool
	// View
	GetBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin
	SpendableCoins(ctx sdk.Context, addr sdk.AccAddress) sdk.Coins
}

// StakingKeeper defines the expected account keeper used for staking operations
type StakingKeeper interface {
	GetParams(ctx sdk.Context) stakingtypes.Params
	GetAllDelegations(ctx sdk.Context) (delegations []stakingtypes.Delegation)
	GetDelegatorDelegations(ctx sdk.Context, delegator sdk.AccAddress, maxRetrieve uint16) (delegations []stakingtypes.Delegation)
}

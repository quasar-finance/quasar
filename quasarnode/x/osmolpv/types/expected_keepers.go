package types

import (
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	qoracletypes "github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/x/auth/types"
)

// AccountKeeper defines the expected account keeper used for simulations (noalias)
type AccountKeeper interface {
	GetAccount(ctx sdk.Context, addr sdk.AccAddress) types.AccountI
	GetModuleAddress(moduleName string) sdk.AccAddress
	// Methods imported from account should be defined here
}

// BankKeeper defines the expected interface needed to retrieve account balances.
type BankKeeper interface {
	SpendableCoins(ctx sdk.Context, addr sdk.AccAddress) sdk.Coins
	// Methods imported from bank should be defined here
	GetAllBalances(ctx sdk.Context, addr sdk.AccAddress) sdk.Coins
	GetBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin
	SendCoinsFromModuleToAccount(ctx sdk.Context, senderModule string, recipientAddr sdk.AccAddress, amt sdk.Coins) error
	SendCoinsFromAccountToModule(ctx sdk.Context, senderAddr sdk.AccAddress, recipientModule string, amt sdk.Coins) error
	SendCoinsFromModuleToModule(ctx sdk.Context, senderModule, recipientModule string, amt sdk.Coins) error
}

type QbankKeeper interface {
	GetUserDenomEpochLockupDepositAmount(ctx sdk.Context,
		uid, denom string, epochday uint64, lockupPeriod qbanktypes.LockupTypes) (val sdk.Coin, found bool)
}

type QoracleKeeper interface {
	GetPoolInfo(ctx sdk.Context, poolId string) (val qoracletypes.PoolInfo, found bool)
	GetPoolRanking(ctx sdk.Context) (val qoracletypes.PoolRanking, found bool)
}

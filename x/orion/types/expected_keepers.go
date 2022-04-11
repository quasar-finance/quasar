package types

import (
	intergammtypes "github.com/abag/quasarnode/x/intergamm/types"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	qoracletypes "github.com/abag/quasarnode/x/qoracle/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/x/auth/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
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
	MintCoins(ctx sdk.Context, moduleName string, amounts sdk.Coins) error
	BurnCoins(ctx sdk.Context, moduleName string, amounts sdk.Coins) error
}

// QbankKeeper defines the expected interface needed by Orion module from qbank
type QbankKeeper interface {
	GetStoreKey() sdk.StoreKey
	GetUserDepositAmt(ctx sdk.Context, uid string) (val qbanktypes.QCoins, found bool)
	GetTotalDeposits(ctx sdk.Context) sdk.Coins
	GetTotalEpochDeposits(ctx sdk.Context, epochday uint64) sdk.Coins
	GetEpochUserDepositAmt(ctx sdk.Context, epochday uint64, uid string) sdk.Coins
	AddUserClaimReward(ctx sdk.Context, uid, vaultID string, coin sdk.Coin)
	AddActualWithdrableAmt(ctx sdk.Context, uid string, coin sdk.Coin)
	AddUserClaimRewards(ctx sdk.Context, uid, vaultID string, coins sdk.Coins)
}

// QoracleKeeper defines the expected interface needed by Orion module from qoracle module
type QoracleKeeper interface {
	GetPoolInfo(ctx sdk.Context, poolId string) (val qoracletypes.PoolInfo, found bool)
	GetPoolRanking(ctx sdk.Context) (val qoracletypes.PoolRanking, found bool)
	GetStablePrice(ctx sdk.Context, denom string) sdk.Dec
}

// IntergammKeeper defines the expected interface needed by Orion module from intergamm module
type IntergammKeeper interface {
	TransmitIbcJoinPoolPacket(
		ctx sdk.Context, packetData intergammtypes.IbcJoinPoolPacketData,
		sourcePort, sourceChannel string, timeoutHeight clienttypes.Height, timeoutTimestamp uint64) error

	TransmitIbcExitPoolPacket(
		ctx sdk.Context, packetData intergammtypes.IbcExitPoolPacketData,
		sourcePort, sourceChannel string,
		timeoutHeight clienttypes.Height, timeoutTimestamp uint64) error

	TransmitIbcWithdrawPacket(
		ctx sdk.Context, packetData intergammtypes.IbcWithdrawPacketData,
		sourcePort, sourceChannel string,
		timeoutHeight clienttypes.Height, timeoutTimestamp uint64) error
}

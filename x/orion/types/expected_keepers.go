package types

import (
	time "time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/x/auth/types"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	connectiontypes "github.com/cosmos/ibc-go/v3/modules/core/03-connection/types"
	gammbalancer "github.com/quasarlabs/quasarnode/osmosis/gamm/pool-models/balancer"
	epochtypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	intergammtypes "github.com/quasarlabs/quasarnode/x/intergamm/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
	qoracletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
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
	GetAllActiveUserDeposits(ctx sdk.Context, todayEpochDay uint64) map[string]sdk.Coins
	GetEpochUserDepositAmt(ctx sdk.Context, epochday uint64, uid string) sdk.Coins
	GetEpochLockupDepositAllUsersAllDenoms(ctx sdk.Context, epochDay uint64, lockupPeriod qbanktypes.LockupTypes) map[string]sdk.Coins
	AddUserClaimReward(ctx sdk.Context, uid, vaultID string, coin sdk.Coin)
	AddActualWithdrawableAmt(ctx sdk.Context, uid string, coin sdk.Coin)
	GetEpochLockupCoins(ctx sdk.Context, epochDay uint64) qbanktypes.EpochLockupCoins
	AddUserClaimRewards(ctx sdk.Context, uid, vaultID string, coins sdk.Coins)
	WhiteListedDenomsInOrion(ctx sdk.Context) (res []qbanktypes.WhiteListedDenomInOrion)
}

// QoracleKeeper defines the expected interface needed by Orion module from qoracle module
type QoracleKeeper interface {
	GetOsmosisPoolsRankedByAPY(ctx sdk.Context, denom string) []qoracletypes.OsmosisPool
	GetOsmosisPool(ctx sdk.Context, id uint64) (qoracletypes.OsmosisPool, bool)
	GetStablePrice(ctx sdk.Context, denom string) (sdk.Dec, bool)
	GetRelativeStablePrice(ctx sdk.Context, denomIn, denomOut string) (sdk.Dec, error)
}

// IntergammKeeper defines the expected interface needed by Orion module from intergamm module
type IntergammKeeper interface {
	RegisterInterchainAccount(ctx sdk.Context, connectionID, owner string) error
	RegisterICAOnZoneId(ctx sdk.Context, zoneId, owner string) error
	RegisterICAOnDenomNativeZone(ctx sdk.Context, denom, owner string) error
	CompleteZoneInfoMap(ctx sdk.Context) (res map[string]intergammtypes.ZoneCompleteInfo)
	QuasarDenomToNativeZoneIdMap(ctx sdk.Context) (res map[string]string)

	IsICARegistered(ctx sdk.Context, connectionID, owner string) (string, bool)
	IsICACreatedOnZoneId(ctx sdk.Context, zoneId, owner string) (string, bool)
	IsICACreatedOnDenomNativeZone(ctx sdk.Context, denom, owner string) (string, bool)
	GetAllConnections(ctx sdk.Context) (connections []connectiontypes.IdentifiedConnection)
	GetChainID(ctx sdk.Context, connectionID string) (string, error)
	GetConnectionId(ctx sdk.Context, inChainID string) (string, bool)

	SendToken(ctx sdk.Context,
		destination_local_zone_id string,
		sender sdk.AccAddress,
		receiver string,
		coin sdk.Coin) (uint64, error)

	TransmitIbcCreatePool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolParams *gammbalancer.PoolParams,
		poolAssets []gammbalancer.PoolAsset,
		futurePoolGovernor string) (uint64, error)

	TransmitIbcJoinPool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		shareOutAmount sdk.Int,
		tokenInMaxs []sdk.Coin) (uint64, error)

	TransmitIbcExitPool(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		poolId uint64,
		shareInAmount sdk.Int,
		tokenOutMins []sdk.Coin) (uint64, error)

	TransmitICATransfer(
		ctx sdk.Context,
		owner string,
		timeoutTimestamp uint64,
		token sdk.Coin,
		finalReceiver string,
		transferTimeoutHeight ibcclienttypes.Height,
		transferTimeoutTimestamp uint64) (uint64, error)

	TransmitIbcLockTokens(
		ctx sdk.Context,
		owner string,
		connectionId string,
		timeoutTimestamp uint64,
		duration time.Duration,
		coins sdk.Coins,
	) (uint64, error)
}

type EpochsKeeper interface {
	GetEpochInfo(ctx sdk.Context, identifier string) epochtypes.EpochInfo
}
